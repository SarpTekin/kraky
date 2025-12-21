//! Kraken WebSocket client

use crate::error::{KrakyError, Result};
use crate::messages::{KrakyMessage, PingRequest, SubscribeRequest, KRAKEN_WS_URL};
use crate::models::{Interval, OHLC, Orderbook, OrderbookUpdate, Ticker, Trade};
use crate::subscriptions::{Subscription, SubscriptionManager, SubscriptionSender};

use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

/// WebSocket connection type
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Command to send to the WebSocket handler
#[derive(Debug)]
enum Command {
    Subscribe(SubscribeRequest),
    Ping,
    Shutdown,
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
    command_tx: mpsc::UnboundedSender<Command>,
    /// Subscription manager
    subscriptions: Arc<RwLock<SubscriptionManager>>,
    /// Managed orderbooks
    orderbooks: Arc<RwLock<HashMap<String, Orderbook>>>,
    /// Connection status
    connected: Arc<RwLock<bool>>,
}

impl KrakyClient {
    /// Connect to Kraken WebSocket API
    /// 
    /// Establishes a WebSocket connection to Kraken's public data API
    /// and starts the message handling loop.
    pub async fn connect() -> Result<Self> {
        Self::connect_with_url(KRAKEN_WS_URL).await
    }

    /// Connect to a custom WebSocket URL (for testing)
    pub async fn connect_with_url(url: &str) -> Result<Self> {
        info!("Connecting to Kraken WebSocket: {}", url);
        
        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket connection established");

        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let subscriptions = Arc::new(RwLock::new(SubscriptionManager::new()));
        let orderbooks = Arc::new(RwLock::new(HashMap::new()));
        let connected = Arc::new(RwLock::new(true));

        // Spawn the WebSocket handler
        let handler = WebSocketHandler {
            subscriptions: Arc::clone(&subscriptions),
            orderbooks: Arc::clone(&orderbooks),
            connected: Arc::clone(&connected),
        };
        
        tokio::spawn(handler.run(ws_stream, command_rx));

        // Spawn heartbeat task
        let heartbeat_tx = command_tx.clone();
        let heartbeat_connected = Arc::clone(&connected);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if !*heartbeat_connected.read() {
                    break;
                }
                if heartbeat_tx.send(Command::Ping).is_err() {
                    break;
                }
            }
        });

        Ok(Self {
            command_tx,
            subscriptions,
            orderbooks,
            connected,
        })
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        *self.connected.read()
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

        // Send subscribe request
        let request = SubscribeRequest::orderbook(vec![pair.to_string()], depth);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to trade updates for a trading pair
    pub async fn subscribe_trades(&self, pair: &str) -> Result<Subscription<Trade>> {
        let (sender, subscription) = SubscriptionSender::new("trade".to_string(), pair.to_string());

        {
            let mut subs = self.subscriptions.write();
            subs.trades.push(sender);
        }

        let request = SubscribeRequest::trades(vec![pair.to_string()]);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to ticker updates for a trading pair
    pub async fn subscribe_ticker(&self, pair: &str) -> Result<Subscription<Ticker>> {
        let (sender, subscription) = SubscriptionSender::new("ticker".to_string(), pair.to_string());

        {
            let mut subs = self.subscriptions.write();
            subs.ticker.push(sender);
        }

        let request = SubscribeRequest::ticker(vec![pair.to_string()]);
        self.command_tx
            .send(Command::Subscribe(request))
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))?;

        Ok(subscription)
    }

    /// Subscribe to OHLC (candlestick) updates for a trading pair
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

    /// Disconnect from the WebSocket
    pub fn disconnect(&self) {
        *self.connected.write() = false;
        let _ = self.command_tx.send(Command::Shutdown);
    }
}

impl Drop for KrakyClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// Internal WebSocket message handler
struct WebSocketHandler {
    subscriptions: Arc<RwLock<SubscriptionManager>>,
    orderbooks: Arc<RwLock<HashMap<String, Orderbook>>>,
    connected: Arc<RwLock<bool>>,
}

impl WebSocketHandler {
    async fn run(self, ws_stream: WsStream, mut command_rx: mpsc::UnboundedReceiver<Command>) {
        let (mut write, mut read) = ws_stream.split();

        loop {
            tokio::select! {
                // Handle incoming WebSocket messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            self.handle_message(&text);
                        }
                        Some(Ok(Message::Close(_))) => {
                            info!("WebSocket connection closed by server");
                            break;
                        }
                        Some(Ok(Message::Ping(data))) => {
                            if let Err(e) = write.send(Message::Pong(data)).await {
                                error!("Failed to send pong: {}", e);
                            }
                        }
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            info!("WebSocket stream ended");
                            break;
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
                        Some(Command::Shutdown) | None => {
                            info!("Shutting down WebSocket handler");
                            break;
                        }
                    }
                }
            }
        }

        *self.connected.write() = false;
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
                        // Parse Kraken error for better diagnostics
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
                KrakyMessage::Orderbook(update) => {
                    // Update managed orderbook state
                    for data in &update.data {
                        let mut orderbooks = self.orderbooks.write();
                        if let Some(orderbook) = orderbooks.get_mut(&data.symbol) {
                            orderbook.apply_update(data);
                        }
                    }
                    
                    // Dispatch to subscriptions
                    self.subscriptions.read().dispatch_orderbook(&update);
                }
                KrakyMessage::Trade(update) => {
                    self.subscriptions.read().dispatch_trade(&update);
                }
                KrakyMessage::Ticker(update) => {
                    self.subscriptions.read().dispatch_ticker(&update);
                }
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

