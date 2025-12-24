//! Kraken WebSocket client

use crate::error::{KrakyError, Result};
use crate::messages::{KrakyMessage, PingRequest, SubscribeRequest, KRAKEN_WS_URL};
use crate::subscriptions::{Subscription, SubscriptionManager, SubscriptionSender};

#[cfg(feature = "orderbook")]
use crate::models::{Orderbook, OrderbookUpdate};
#[cfg(feature = "trades")]
use crate::models::Trade;
#[cfg(feature = "ticker")]
use crate::models::Ticker;
#[cfg(feature = "ohlc")]
use crate::models::{Interval, OHLC};

use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{protocol::WebSocketConfig, Message},
    Connector, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

/// Connection event emitted by the client
///
/// Only available when the `events` feature is enabled.
#[cfg(feature = "events")]
#[derive(Debug, Clone)]
pub enum ConnectionEvent {
    /// Successfully connected
    Connected,
    /// Disconnected (with optional reason)
    Disconnected(Option<String>),
    /// Attempting to reconnect (attempt number)
    Reconnecting(u32),
    /// Reconnection successful
    Reconnected,
    /// Reconnection failed (attempt number, error message)
    ReconnectFailed(u32, String),
    /// Max reconnection attempts reached
    ReconnectExhausted,
}

/// Connection state for the WebSocket client
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectionState {
    /// Not connected
    Disconnected = 0,
    /// Attempting to connect
    Connecting = 1,
    /// Connected and ready
    Connected = 2,
    /// Reconnecting after disconnect
    Reconnecting = 3,
}

impl From<u8> for ConnectionState {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Disconnected,
            1 => Self::Connecting,
            2 => Self::Connected,
            3 => Self::Reconnecting,
            _ => Self::Disconnected,
        }
    }
}

/// Configuration for automatic reconnection
///
/// Only available when the `reconnect` feature is enabled.
#[cfg(feature = "reconnect")]
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Whether to automatically reconnect on disconnect
    pub enabled: bool,
    /// Initial delay before first reconnection attempt
    pub initial_delay: Duration,
    /// Maximum delay between reconnection attempts
    pub max_delay: Duration,
    /// Multiplier for exponential backoff (e.g., 2.0 doubles the delay each time)
    pub backoff_multiplier: f64,
    /// Maximum number of reconnection attempts (None = unlimited)
    pub max_attempts: Option<u32>,
}

#[cfg(feature = "reconnect")]
impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            max_attempts: None, // Unlimited retries
        }
    }
}

#[cfg(feature = "reconnect")]
impl ReconnectConfig {
    /// Create a config with reconnection disabled
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }

    /// Create a config with aggressive reconnection (for low-latency needs)
    pub fn aggressive() -> Self {
        Self {
            enabled: true,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            max_attempts: None,
        }
    }

    /// Create a config with conservative reconnection (to avoid rate limiting)
    pub fn conservative() -> Self {
        Self {
            enabled: true,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            max_attempts: Some(10),
        }
    }

    /// Calculate delay for a given attempt number
    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay_ms = self.initial_delay.as_millis() as f64
            * self.backoff_multiplier.powi(attempt as i32);
        let delay = Duration::from_millis(delay_ms as u64);
        delay.min(self.max_delay)
    }
}

/// Stored subscription info for re-subscription after reconnect
#[cfg(feature = "reconnect")]
#[derive(Debug, Clone)]
enum StoredSubscription {
    #[cfg(feature = "orderbook")]
    Orderbook { pair: String, depth: u32 },
    #[cfg(feature = "trades")]
    Trades { pair: String },
    #[cfg(feature = "ticker")]
    Ticker { pair: String },
    #[cfg(feature = "ohlc")]
    OHLC { pair: String, interval: u32 },
}

/// WebSocket connection type
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Command to send to the WebSocket handler
#[derive(Debug, Clone)]
enum Command {
    Subscribe(SubscribeRequest),
    Ping,
    Shutdown,
    /// Trigger reconnection
    Reconnect,
    /// Send a raw JSON message (for trading, etc.)
    #[cfg(feature = "trading")]
    RawMessage(String),
}

/// Kraken WebSocket client
/// 
/// Provides a high-level interface for connecting to Kraken's WebSocket API
/// and subscribing to market data streams.
/// 
/// # Example
/// 
/// ```no_run
/// use kraky::KrakyClient;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = KrakyClient::connect().await?;
///     
///     let mut orderbook_sub = client.subscribe_orderbook("BTC/USD", 10).await?;
///     
///     while let Some(update) = orderbook_sub.next().await {
///         println!("Orderbook update: {:?}", update);
///     }
///     
///     Ok(())
/// }
/// ```
pub struct KrakyClient {
    /// Command sender for the WebSocket handler
    command_tx: tokio::sync::mpsc::UnboundedSender<Command>,
    /// Subscription manager
    subscriptions: Arc<RwLock<SubscriptionManager>>,
    /// Managed orderbooks
    #[cfg(feature = "orderbook")]
    orderbooks: Arc<RwLock<HashMap<String, Orderbook>>>,
    /// Connection state (lock-free atomic)
    state: Arc<AtomicU8>,
    /// Reconnection configuration
    #[cfg(feature = "reconnect")]
    reconnect_config: Arc<ReconnectConfig>,
    /// Stored subscriptions for re-subscription after reconnect
    #[cfg(feature = "reconnect")]
    stored_subscriptions: Arc<RwLock<Vec<StoredSubscription>>>,
    /// URL for reconnection
    url: Arc<String>,
    /// Shutdown flag
    shutdown: Arc<AtomicBool>,
    /// Connection event broadcaster
    #[cfg(feature = "events")]
    event_tx: Arc<RwLock<Option<mpsc::Sender<ConnectionEvent>>>>,
}

impl KrakyClient {
    /// Connect to Kraken WebSocket API with default reconnection settings
    /// 
    /// Establishes a WebSocket connection to Kraken's public data API
    /// and starts the message handling loop with automatic reconnection.
    pub async fn connect() -> Result<Self> {
        Self::connect_with_config(KRAKEN_WS_URL, ReconnectConfig::default()).await
    }

    /// Connect with a custom reconnection configuration
    pub async fn connect_with_reconnect(config: ReconnectConfig) -> Result<Self> {
        Self::connect_with_config(KRAKEN_WS_URL, config).await
    }

    /// Connect to a custom WebSocket URL (for testing)
    pub async fn connect_with_url(url: &str) -> Result<Self> {
        Self::connect_with_config(url, ReconnectConfig::default()).await
    }

    /// Connect with full configuration options
    pub async fn connect_with_config(url: &str, reconnect_config: ReconnectConfig) -> Result<Self> {
        let state = Arc::new(AtomicU8::new(ConnectionState::Connecting as u8));
        let shutdown = Arc::new(AtomicBool::new(false));
        let url = Arc::new(url.to_string());
        let reconnect_config = Arc::new(reconnect_config);
        let stored_subscriptions = Arc::new(RwLock::new(Vec::new()));
        let subscriptions = Arc::new(RwLock::new(SubscriptionManager::new()));
        #[cfg(feature = "orderbook")]
        let orderbooks = Arc::new(RwLock::new(HashMap::new()));
        let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();
        let event_tx: Arc<RwLock<Option<mpsc::Sender<ConnectionEvent>>>> = Arc::new(RwLock::new(None));

        // Initial connection
        let ws_stream = Self::create_connection(&url).await?;
        state.store(ConnectionState::Connected as u8, Ordering::SeqCst);
        info!("WebSocket connection established (TCP_NODELAY enabled)");

        // Spawn the connection manager task
        let manager = ConnectionManager {
            subscriptions: Arc::clone(&subscriptions),
            #[cfg(feature = "orderbook")]
            orderbooks: Arc::clone(&orderbooks),
            state: Arc::clone(&state),
            reconnect_config: Arc::clone(&reconnect_config),
            stored_subscriptions: Arc::clone(&stored_subscriptions),
            url: Arc::clone(&url),
            shutdown: Arc::clone(&shutdown),
            event_tx: Arc::clone(&event_tx),
        };
        
        tokio::spawn(manager.run(ws_stream, command_rx));

        // Spawn heartbeat task
        let heartbeat_tx = command_tx.clone();
        let heartbeat_state = Arc::clone(&state);
        let heartbeat_shutdown = Arc::clone(&shutdown);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if heartbeat_shutdown.load(Ordering::Relaxed) {
                    break;
                }
                let current_state = ConnectionState::from(heartbeat_state.load(Ordering::Relaxed));
                if current_state == ConnectionState::Connected {
                    if heartbeat_tx.send(Command::Ping).is_err() {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            command_tx,
            subscriptions,
            #[cfg(feature = "orderbook")]
            orderbooks,
            state,
            reconnect_config,
            stored_subscriptions,
            url,
            shutdown,
            event_tx,
        })
    }

    /// Create a new WebSocket connection (used for initial connect and reconnect)
    async fn create_connection(url: &str) -> Result<WsStream> {
        info!("Connecting to Kraken WebSocket: {}", url);
        
        // Configure WebSocket for low latency
        let ws_config = WebSocketConfig {
            write_buffer_size: 0,
            max_message_size: Some(16 * 1024 * 1024),
            max_frame_size: Some(16 * 1024 * 1024),
            accept_unmasked_frames: false,
            ..Default::default()
        };
        
        let connector = Connector::NativeTls(
            native_tls::TlsConnector::new()
                .map_err(|e| KrakyError::Connection(
                    tokio_tungstenite::tungstenite::Error::Tls(e.into())
                ))?
        );
        
        let (ws_stream, _) = connect_async_tls_with_config(
            url,
            Some(ws_config),
            false,
            Some(connector),
        ).await?;
        
        Ok(ws_stream)
    }

    /// Get the current connection state
    pub fn connection_state(&self) -> ConnectionState {
        ConnectionState::from(self.state.load(Ordering::Relaxed))
    }

    /// Check if the client is connected (lock-free)
    pub fn is_connected(&self) -> bool {
        self.connection_state() == ConnectionState::Connected
    }

    /// Check if reconnection is in progress
    pub fn is_reconnecting(&self) -> bool {
        self.connection_state() == ConnectionState::Reconnecting
    }

    /// Get the reconnection configuration
    pub fn reconnect_config(&self) -> &ReconnectConfig {
        &self.reconnect_config
    }

    /// Get the WebSocket URL this client is connected to
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Subscribe to connection events
    ///
    /// Returns a receiver that will receive connection state changes.
    /// Only one subscriber is supported at a time; calling this again
    /// replaces the previous subscriber.
    ///
    /// Only available when the `events` feature is enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut events = client.subscribe_events();
    ///
    /// tokio::spawn(async move {
    ///     while let Some(event) = events.recv().await {
    ///         match event {
    ///             ConnectionEvent::Connected => println!("Connected!"),
    ///             ConnectionEvent::Disconnected(reason) => println!("Lost: {:?}", reason),
    ///             ConnectionEvent::Reconnecting(n) => println!("Reconnecting #{}", n),
    ///             ConnectionEvent::Reconnected => println!("Reconnected!"),
    ///             ConnectionEvent::ReconnectFailed(n, e) => println!("Failed #{}: {}", n, e),
    ///             ConnectionEvent::ReconnectExhausted => println!("Gave up reconnecting"),
    ///         }
    ///     }
    /// });
    /// ```
    #[cfg(feature = "events")]
    pub fn subscribe_events(&self) -> mpsc::Receiver<ConnectionEvent> {
        let (tx, rx) = mpsc::channel(100);
        *self.event_tx.write() = Some(tx);
        rx
    }

    /// Subscribe to orderbook updates for a trading pair
    ///
    /// # Arguments
    ///
    /// * `pair` - Trading pair symbol (e.g., "BTC/USD")
    /// * `depth` - Number of price levels (10, 25, 100, 500, or 1000)
    ///
    /// # Returns
    ///
    /// A subscription stream that yields orderbook updates
    ///
    /// Only available when the `orderbook` feature is enabled.
    #[cfg(feature = "orderbook")]
    pub async fn subscribe_orderbook(
        &self,
        pair: &str,
        depth: u32,
    ) -> Result<Subscription<OrderbookUpdate>> {
        let (sender, subscription) = SubscriptionSender::new("book".to_string(), pair.to_string());
        
        // Initialize orderbook state
        {
            let mut orderbooks = self.orderbooks.write();
            orderbooks.insert(pair.to_string(), Orderbook::new(pair.to_string()));
        }

        // Add subscription
        {
            let mut subs = self.subscriptions.write();
            subs.orderbook.push(sender);
        }

        // Store for reconnection
        {
            let mut stored = self.stored_subscriptions.write();
            stored.push(StoredSubscription::Orderbook { 
                pair: pair.to_string(), 
                depth 
            });
        }

        // Send subscribe request
        let request = SubscribeRequest::orderbook(vec![pair.to_string()], depth);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to trade updates for a trading pair
    ///
    /// Only available when the `trades` feature is enabled.
    #[cfg(feature = "trades")]
    pub async fn subscribe_trades(&self, pair: &str) -> Result<Subscription<Trade>> {
        let (sender, subscription) = SubscriptionSender::new("trade".to_string(), pair.to_string());

        {
            let mut subs = self.subscriptions.write();
            subs.trades.push(sender);
        }

        // Store for reconnection
        {
            let mut stored = self.stored_subscriptions.write();
            stored.push(StoredSubscription::Trades { 
                pair: pair.to_string() 
            });
        }

        let request = SubscribeRequest::trades(vec![pair.to_string()]);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to ticker updates for a trading pair
    ///
    /// Only available when the `ticker` feature is enabled.
    #[cfg(feature = "ticker")]
    pub async fn subscribe_ticker(&self, pair: &str) -> Result<Subscription<Ticker>> {
        let (sender, subscription) = SubscriptionSender::new("ticker".to_string(), pair.to_string());

        {
            let mut subs = self.subscriptions.write();
            subs.ticker.push(sender);
        }

        // Store for reconnection
        {
            let mut stored = self.stored_subscriptions.write();
            stored.push(StoredSubscription::Ticker { 
                pair: pair.to_string() 
            });
        }

        let request = SubscribeRequest::ticker(vec![pair.to_string()]);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to OHLC (candlestick) updates for a trading pair
    ///
    /// Only available when the `ohlc` feature is enabled.
    #[cfg(feature = "ohlc")]
    pub async fn subscribe_ohlc(
        &self,
        pair: &str,
        interval: Interval,
    ) -> Result<Subscription<OHLC>> {
        let (sender, subscription) = SubscriptionSender::new("ohlc".to_string(), pair.to_string());

        {
            let mut subs = self.subscriptions.write();
            subs.ohlc.push(sender);
        }

        // Store for reconnection
        {
            let mut stored = self.stored_subscriptions.write();
            stored.push(StoredSubscription::OHLC { 
                pair: pair.to_string(), 
                interval: interval.minutes() 
            });
        }

        let request = SubscribeRequest::ohlc(vec![pair.to_string()], interval.minutes());
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Get the current orderbook for a trading pair
    pub fn get_orderbook(&self, pair: &str) -> Option<Orderbook> {
        self.orderbooks.read().get(pair).cloned()
    }

    /// Check if the orderbook for a pair has a valid checksum
    /// 
    /// Returns `None` if no orderbook exists for the pair.
    /// Returns `Some(true)` if the last checksum validation passed.
    /// Returns `Some(false)` if the orderbook might be corrupted.
    /// 
    /// Only available when the `checksum` feature is enabled.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if client.is_orderbook_valid("BTC/USD") == Some(false) {
    ///     // Orderbook is corrupted, trigger reconnect
    ///     client.reconnect()?;
    /// }
    /// ```
    #[cfg(feature = "checksum")]
    pub fn is_orderbook_valid(&self, pair: &str) -> Option<bool> {
        self.orderbooks.read().get(pair).map(|ob| ob.checksum_valid)
    }

    /// Validate all orderbooks and reconnect if any are corrupted
    /// 
    /// Returns the number of corrupted orderbooks found.
    /// If any are corrupted, a reconnection is triggered automatically.
    /// 
    /// Only available when the `checksum` feature is enabled.
    #[cfg(feature = "checksum")]
    pub fn validate_orderbooks_and_reconnect(&self) -> Result<usize> {
        let corrupted: Vec<String> = self.orderbooks
            .read()
            .iter()
            .filter(|(_, ob)| !ob.checksum_valid)
            .map(|(pair, _)| pair.clone())
            .collect();

        let count = corrupted.len();
        
        if count > 0 {
            warn!(
                "Found {} corrupted orderbook(s): {:?}. Triggering reconnect.",
                count, corrupted
            );
            self.reconnect()?;
        }

        Ok(count)
    }

    // ============================================================================
    // Trading Methods (requires 'trading' feature)
    // ============================================================================

    /// Place an order
    ///
    /// Requires authentication credentials to be set up.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use kraky::{OrderParams, Credentials};
    ///
    /// let creds = Credentials::new("api_key", "api_secret");
    /// let order = OrderParams::market_buy("BTC/USD", 0.1);
    /// let response = client.place_order(&creds, order).await?;
    /// println!("Order placed: {}", response.order_id);
    /// ```
    #[cfg(feature = "trading")]
    pub async fn place_order(
        &self,
        credentials: &crate::auth::Credentials,
        params: crate::models::OrderParams,
    ) -> Result<crate::models::OrderResponse> {
        use crate::models::OrderResponse;

        // Generate authentication token
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let token = credentials.generate_token(nonce)?;

        // Build request message
        let request = serde_json::json!({
            "method": "add_order",
            "params": {
                "token": token,
                "symbol": params.symbol,
                "side": params.side,
                "order_type": params.order_type,
                "order_qty": params.order_qty,
                "limit_price": params.limit_price,
                "trigger_price": params.trigger_price,
                "time_in_force": params.time_in_force,
                "post_only": params.post_only,
                "reduce_only": params.reduce_only,
                "stp": params.stp,
                "cl_ord_id": params.cl_ord_id,
                "validate": params.validate,
            }
        });

        // Send request and wait for response
        // Note: This is a simplified implementation
        // A full implementation would need proper response handling
        self.command_tx
            .send(Command::RawMessage(request.to_string()))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        // For now, return a placeholder
        // A complete implementation would parse the actual response
        Ok(OrderResponse {
            order_id: "pending".to_string(),
            cl_ord_id: params.cl_ord_id,
            order_status: crate::models::OrderStatus::Pending,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Cancel an order by ID
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = client.cancel_order(&creds, "order-id-123").await?;
    /// ```
    #[cfg(feature = "trading")]
    pub async fn cancel_order(
        &self,
        credentials: &crate::auth::Credentials,
        order_id: impl Into<String>,
    ) -> Result<crate::models::CancelOrderResponse> {
        use crate::models::CancelOrderResponse;

        let order_id = order_id.into();

        // Generate authentication token
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let token = credentials.generate_token(nonce)?;

        // Build request message
        let request = serde_json::json!({
            "method": "cancel_order",
            "params": {
                "token": token,
                "order_id": [order_id.clone()],
            }
        });

        // Send request
        self.command_tx
            .send(Command::RawMessage(request.to_string()))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        // Return placeholder
        Ok(CancelOrderResponse {
            order_id,
            success: true,
        })
    }

    /// Cancel all open orders
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = client.cancel_all_orders(&creds).await?;
    /// println!("Cancelled {} orders", response.count);
    /// ```
    #[cfg(feature = "trading")]
    pub async fn cancel_all_orders(
        &self,
        credentials: &crate::auth::Credentials,
    ) -> Result<crate::models::CancelAllResponse> {
        use crate::models::CancelAllResponse;

        // Generate authentication token
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let token = credentials.generate_token(nonce)?;

        // Build request message
        let request = serde_json::json!({
            "method": "cancel_all",
            "params": {
                "token": token,
            }
        });

        // Send request
        self.command_tx
            .send(Command::RawMessage(request.to_string()))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        // Return placeholder
        Ok(CancelAllResponse {
            count: 0,
        })
    }

    /// Amend (modify) an existing order
    ///
    /// # Example
    ///
    /// ```ignore
    /// use kraky::AmendOrderParams;
    ///
    /// let amend = AmendOrderParams {
    ///     order_id: "order-123".to_string(),
    ///     order_qty: Some(0.2),
    ///     limit_price: Some(51000.0),
    ///     trigger_price: None,
    /// };
    /// let response = client.amend_order(&creds, amend).await?;
    /// ```
    #[cfg(feature = "trading")]
    pub async fn amend_order(
        &self,
        credentials: &crate::auth::Credentials,
        params: crate::models::AmendOrderParams,
    ) -> Result<crate::models::AmendOrderResponse> {
        use crate::models::AmendOrderResponse;

        // Generate authentication token
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let token = credentials.generate_token(nonce)?;

        // Build request message
        let request = serde_json::json!({
            "method": "amend_order",
            "params": {
                "token": token,
                "order_id": params.order_id.clone(),
                "order_qty": params.order_qty,
                "limit_price": params.limit_price,
                "trigger_price": params.trigger_price,
            }
        });

        // Send request
        self.command_tx
            .send(Command::RawMessage(request.to_string()))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        // Return placeholder
        Ok(AmendOrderResponse {
            order_id: params.order_id,
            success: true,
            error: None,
        })
    }

    /// Disconnect from the WebSocket (lock-free)
    /// 
    /// This will stop reconnection attempts and close the connection.
    pub fn disconnect(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.state.store(ConnectionState::Disconnected as u8, Ordering::SeqCst);
        let _ = self.command_tx.send(Command::Shutdown);
    }

    /// Manually trigger a reconnection
    /// 
    /// Useful if you want to force a fresh connection.
    pub fn reconnect(&self) -> Result<()> {
        if self.shutdown.load(Ordering::Relaxed) {
            return Err(KrakyError::ChannelSend("Client is shut down".to_string()));
        }
        self.command_tx
            .send(Command::Reconnect)
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))
    }
}

impl Drop for KrakyClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// Connection manager that handles WebSocket messages and reconnection
struct ConnectionManager {
    subscriptions: Arc<RwLock<SubscriptionManager>>,
    #[cfg(feature = "orderbook")]
    orderbooks: Arc<RwLock<HashMap<String, Orderbook>>>,
    state: Arc<AtomicU8>,
    reconnect_config: Arc<ReconnectConfig>,
    stored_subscriptions: Arc<RwLock<Vec<StoredSubscription>>>,
    url: Arc<String>,
    shutdown: Arc<AtomicBool>,
    event_tx: Arc<RwLock<Option<mpsc::Sender<ConnectionEvent>>>>,
}

impl ConnectionManager {
    /// Emit a connection event to subscribers
    fn emit_event(&self, event: ConnectionEvent) {
        if let Some(tx) = self.event_tx.read().as_ref() {
            // Use try_send to avoid blocking; drop event if channel is full
            let _ = tx.try_send(event);
        }
    }

    async fn run(
        self,
        initial_stream: WsStream,
        mut command_rx: tokio::sync::mpsc::UnboundedReceiver<Command>,
    ) {
        let mut ws_stream = Some(initial_stream);
        let mut reconnect_attempt = 0u32;
        let mut pending_commands: Vec<Command> = Vec::new();
        
        // Emit initial connected event
        self.emit_event(ConnectionEvent::Connected);

        loop {
            // Check shutdown flag
            if self.shutdown.load(Ordering::Relaxed) {
                info!("Connection manager shutting down");
                self.emit_event(ConnectionEvent::Disconnected(Some("Shutdown requested".to_string())));
                break;
            }

            // If we have a connection, run the message loop
            if let Some(stream) = ws_stream.take() {
                let disconnect_reason = self.run_message_loop(
                    stream,
                    &mut command_rx,
                    &mut pending_commands,
                ).await;

                let disconnect_msg = match &disconnect_reason {
                    DisconnectReason::Shutdown => {
                        info!("WebSocket handler shut down");
                        self.emit_event(ConnectionEvent::Disconnected(Some("Shutdown".to_string())));
                        break;
                    }
                    DisconnectReason::ServerClose => {
                        warn!("Server closed connection");
                        Some("Server closed connection".to_string())
                    }
                    DisconnectReason::Error(e) => {
                        error!("WebSocket error: {}", e);
                        Some(e.clone())
                    }
                    DisconnectReason::StreamEnded => {
                        warn!("WebSocket stream ended unexpectedly");
                        Some("Stream ended".to_string())
                    }
                    DisconnectReason::ManualReconnect => {
                        info!("Manual reconnection requested");
                        reconnect_attempt = 0; // Reset attempts for manual reconnect
                        None
                    }
                };

                // Emit disconnect event (unless it's a manual reconnect)
                if disconnect_msg.is_some() {
                    self.emit_event(ConnectionEvent::Disconnected(disconnect_msg));
                }

                // Should we reconnect?
                if !self.reconnect_config.enabled || self.shutdown.load(Ordering::Relaxed) {
                    self.state.store(ConnectionState::Disconnected as u8, Ordering::SeqCst);
                    break;
                }

                // Check max attempts
                if let Some(max) = self.reconnect_config.max_attempts {
                    if reconnect_attempt >= max {
                        error!("Max reconnection attempts ({}) reached, giving up", max);
                        self.emit_event(ConnectionEvent::ReconnectExhausted);
                        self.state.store(ConnectionState::Disconnected as u8, Ordering::SeqCst);
                        break;
                    }
                }

                // Attempt reconnection
                self.state.store(ConnectionState::Reconnecting as u8, Ordering::SeqCst);
                self.emit_event(ConnectionEvent::Reconnecting(reconnect_attempt + 1));
                
                let delay = self.reconnect_config.delay_for_attempt(reconnect_attempt);
                info!(
                    "Reconnecting in {:?} (attempt {}/{})",
                    delay,
                    reconnect_attempt + 1,
                    self.reconnect_config.max_attempts.map(|m| m.to_string()).unwrap_or_else(|| "∞".to_string())
                );
                
                tokio::time::sleep(delay).await;

                // Check shutdown again after sleep
                if self.shutdown.load(Ordering::Relaxed) {
                    self.emit_event(ConnectionEvent::Disconnected(Some("Shutdown during reconnect".to_string())));
                    break;
                }

                match KrakyClient::create_connection(&self.url).await {
                    Ok(new_stream) => {
                        info!("Reconnection successful!");
                        self.state.store(ConnectionState::Connected as u8, Ordering::SeqCst);
                        self.emit_event(ConnectionEvent::Reconnected);
                        reconnect_attempt = 0;
                        ws_stream = Some(new_stream);

                        // Re-subscribe to all stored subscriptions
                        self.resubscribe_all(&mut pending_commands);
                    }
                    Err(e) => {
                        let err_msg = e.to_string();
                        warn!("Reconnection attempt {} failed: {}", reconnect_attempt + 1, err_msg);
                        self.emit_event(ConnectionEvent::ReconnectFailed(reconnect_attempt + 1, err_msg));
                        reconnect_attempt += 1;
                        ws_stream = None;
                    }
                }
            } else {
                // No connection, wait before retrying
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }

        self.state.store(ConnectionState::Disconnected as u8, Ordering::SeqCst);
    }

    fn resubscribe_all(&self, pending_commands: &mut Vec<Command>) {
        let subs = self.stored_subscriptions.read();
        info!("Re-subscribing to {} subscriptions", subs.len());
        
        for sub in subs.iter() {
            let request = match sub {
                #[cfg(feature = "orderbook")]
                StoredSubscription::Orderbook { pair, depth } => {
                    // Reset orderbook state for fresh snapshot
                    {
                        let mut orderbooks = self.orderbooks.write();
                        if let Some(ob) = orderbooks.get_mut(pair) {
                            *ob = Orderbook::new(pair.clone());
                        }
                    }
                    SubscribeRequest::orderbook(vec![pair.clone()], *depth)
                }
                #[cfg(feature = "trades")]
                StoredSubscription::Trades { pair } => {
                    SubscribeRequest::trades(vec![pair.clone()])
                }
                #[cfg(feature = "ticker")]
                StoredSubscription::Ticker { pair } => {
                    SubscribeRequest::ticker(vec![pair.clone()])
                }
                #[cfg(feature = "ohlc")]
                StoredSubscription::OHLC { pair, interval } => {
                    SubscribeRequest::ohlc(vec![pair.clone()], *interval)
                }
            };
            pending_commands.push(Command::Subscribe(request));
        }
    }

    async fn run_message_loop(
        &self,
        ws_stream: WsStream,
        command_rx: &mut tokio::sync::mpsc::UnboundedReceiver<Command>,
        pending_commands: &mut Vec<Command>,
    ) -> DisconnectReason {
        let (mut write, mut read) = ws_stream.split();

        // Send any pending commands (e.g., re-subscriptions)
        for cmd in pending_commands.drain(..) {
            if let Command::Subscribe(request) = cmd {
                if let Ok(json) = serde_json::to_string(&request) {
                    debug!("Sending pending subscribe: {}", json);
                    if let Err(e) = write.send(Message::Text(json)).await {
                        error!("Failed to send pending subscribe: {}", e);
                    }
                }
            }
        }

        loop {
            tokio::select! {
                // Handle incoming WebSocket messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            self.handle_message(&text);
                        }
                        Some(Ok(Message::Close(_))) => {
                            return DisconnectReason::ServerClose;
                        }
                        Some(Ok(Message::Ping(data))) => {
                            if let Err(e) = write.send(Message::Pong(data)).await {
                                error!("Failed to send pong: {}", e);
                            }
                        }
                        Some(Err(e)) => {
                            return DisconnectReason::Error(e.to_string());
                        }
                        None => {
                            return DisconnectReason::StreamEnded;
                        }
                        _ => {}
                    }
                }

                // Handle outgoing commands
                cmd = command_rx.recv() => {
                    match cmd {
                        Some(Command::Subscribe(request)) => {
                            match serde_json::to_string(&request) {
                                Ok(json) => {
                                    debug!("Sending subscribe: {}", json);
                                    if let Err(e) = write.send(Message::Text(json)).await {
                                        error!("Failed to send subscribe: {}", e);
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to serialize subscribe request: {}", e);
                                }
                            }
                        }
                        Some(Command::Ping) => {
                            let ping = PingRequest::default();
                            if let Ok(json) = serde_json::to_string(&ping) {
                                if let Err(e) = write.send(Message::Text(json)).await {
                                    error!("Failed to send ping: {}", e);
                                }
                            }
                        }
                        Some(Command::Reconnect) => {
                            return DisconnectReason::ManualReconnect;
                        }
                        #[cfg(feature = "trading")]
                        Some(Command::RawMessage(json)) => {
                            debug!("Sending raw message: {}", json);
                            if let Err(e) = write.send(Message::Text(json)).await {
                                error!("Failed to send raw message: {}", e);
                            }
                        }
                        Some(Command::Shutdown) | None => {
                            return DisconnectReason::Shutdown;
                        }
                    }
                }
            }
        }
    }

    fn handle_message(&self, text: &str) {
        match KrakyMessage::parse(text) {
            Ok(msg) => match msg {
                KrakyMessage::SystemStatus(status) => {
                    if let Some(data) = status.data.first() {
                        info!(
                            "Connected to Kraken API v{} (system: {})",
                            data.api_version, data.system
                        );
                    }
                }
                KrakyMessage::Heartbeat => {
                    debug!("Received heartbeat");
                }
                KrakyMessage::Pong { req_id } => {
                    debug!("Received pong (req_id: {:?})", req_id);
                }
                KrakyMessage::SubscriptionStatus { success, channel, symbol, error } => {
                    if success {
                        info!("Subscribed to {} for {:?}", channel, symbol);
                    } else if let Some(err_str) = error {
                        let parsed = crate::error::KrakenApiError::parse(&err_str);
                        if parsed.is_retryable() {
                            warn!(
                                "Subscription failed for {} (retryable): [{}:{}] {}",
                                channel, parsed.severity, parsed.category, parsed.message
                            );
                        } else if parsed.is_invalid_pair() {
                            error!(
                                "Invalid trading pair for {}: {}",
                                channel, parsed.message
                            );
                        } else if parsed.is_rate_limited() {
                            warn!("Rate limited on {} subscription", channel);
                        } else {
                            warn!(
                                "Subscription failed for {}: [{}:{}] {}",
                                channel, parsed.severity, parsed.category, parsed.message
                            );
                        }
                    } else {
                        warn!("Subscription failed for {}: unknown error", channel);
                    }
                }
                #[cfg(feature = "orderbook")]
                KrakyMessage::Orderbook(update) => {
                    for data in &update.data {
                        let mut orderbooks = self.orderbooks.write();
                        if let Some(orderbook) = orderbooks.get_mut(&data.symbol) {
                            orderbook.apply_update(data);
                        }
                    }
                    self.subscriptions.read().dispatch_orderbook(&update);
                }
                #[cfg(feature = "trades")]
                KrakyMessage::Trade(update) => {
                    self.subscriptions.read().dispatch_trade(&update);
                }
                #[cfg(feature = "ticker")]
                KrakyMessage::Ticker(update) => {
                    self.subscriptions.read().dispatch_ticker(&update);
                }
                #[cfg(feature = "ohlc")]
                KrakyMessage::OHLC(update) => {
                    self.subscriptions.read().dispatch_ohlc(&update);
                }
                KrakyMessage::Unknown(value) => {
                    debug!("Unknown message: {}", value);
                }
            },
            Err(e) => {
                warn!("Failed to parse message: {} - {}", e, text);
            }
        }
    }
}

/// Reason for WebSocket disconnection
#[derive(Debug)]
enum DisconnectReason {
    Shutdown,
    ServerClose,
    Error(String),
    StreamEnded,
    ManualReconnect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconnect_config_default() {
        let config = ReconnectConfig::default();
        assert!(config.enabled);
        assert_eq!(config.initial_delay, Duration::from_millis(500));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.backoff_multiplier, 2.0);
        assert_eq!(config.max_attempts, None);
    }

    #[test]
    fn test_reconnect_config_disabled() {
        let config = ReconnectConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_reconnect_config_aggressive() {
        let config = ReconnectConfig::aggressive();
        assert!(config.enabled);
        assert_eq!(config.initial_delay, Duration::from_millis(100));
        assert_eq!(config.max_delay, Duration::from_secs(5));
        assert_eq!(config.backoff_multiplier, 1.5);
    }

    #[test]
    fn test_reconnect_config_conservative() {
        let config = ReconnectConfig::conservative();
        assert!(config.enabled);
        assert_eq!(config.initial_delay, Duration::from_secs(1));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.max_attempts, Some(10));
    }

    #[test]
    fn test_exponential_backoff() {
        let config = ReconnectConfig::default();
        
        // First attempt: 500ms
        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(500));
        
        // Second attempt: 1000ms
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(1000));
        
        // Third attempt: 2000ms
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(2000));
        
        // Should cap at max_delay
        assert_eq!(config.delay_for_attempt(10), Duration::from_secs(30));
    }

    #[test]
    fn test_connection_state_conversion() {
        assert_eq!(ConnectionState::from(0), ConnectionState::Disconnected);
        assert_eq!(ConnectionState::from(1), ConnectionState::Connecting);
        assert_eq!(ConnectionState::from(2), ConnectionState::Connected);
        assert_eq!(ConnectionState::from(3), ConnectionState::Reconnecting);
        assert_eq!(ConnectionState::from(255), ConnectionState::Disconnected); // Invalid -> Disconnected
    }
}

