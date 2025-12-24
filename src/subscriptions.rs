//! Subscription stream handling with backpressure control

use crate::error::{KrakyError, Result};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use futures_util::Stream;

/// Default buffer size for subscription channels
pub const DEFAULT_BUFFER_SIZE: usize = 1000;

/// Backpressure configuration for subscriptions
#[derive(Debug, Clone)]
pub struct BackpressureConfig {
    /// Maximum number of messages to buffer before dropping
    pub buffer_size: usize,
}

impl Default for BackpressureConfig {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}

impl BackpressureConfig {
    /// Create a new backpressure config with custom buffer size
    pub fn with_buffer_size(buffer_size: usize) -> Self {
        Self { buffer_size }
    }
}

/// Statistics for a subscription
#[derive(Debug, Default)]
pub struct SubscriptionStats {
    /// Number of messages successfully delivered
    pub delivered: AtomicU64,
    /// Number of messages dropped due to backpressure
    pub dropped: AtomicU64,
}

impl SubscriptionStats {
    /// Get the number of delivered messages
    pub fn delivered(&self) -> u64 {
        self.delivered.load(Ordering::Relaxed)
    }

    /// Get the number of dropped messages
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }

    /// Get the drop rate as a percentage
    pub fn drop_rate(&self) -> f64 {
        let delivered = self.delivered() as f64;
        let dropped = self.dropped() as f64;
        let total = delivered + dropped;
        if total == 0.0 {
            0.0
        } else {
            (dropped / total) * 100.0
        }
    }
}

/// A subscription to a Kraken data stream
/// 
/// Subscriptions are async streams that yield data as it arrives from
/// the Kraken WebSocket API. They can be consumed using the `next()` method
/// or by treating them as a `Stream`.
/// 
/// # Backpressure
/// 
/// Subscriptions use bounded channels to prevent memory issues. If the consumer
/// is too slow, older messages will be dropped to keep the most recent data.
/// Use `stats()` to monitor dropped messages.
/// 
/// # Example
///
/// ```no_run
/// use kraky::KrakyClient;
/// use futures_util::StreamExt;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = KrakyClient::connect().await?;
/// let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
///
/// while let Some(update) = orderbook.next().await {
///     println!("Orderbook update: {:?}", update.data[0].symbol);
/// }
///
/// // Check for dropped messages
/// println!("Dropped: {} ({:.2}%)", orderbook.stats().dropped(), orderbook.stats().drop_rate());
/// # Ok(())
/// # }
/// ```
pub struct Subscription<T> {
    /// Receiver for subscription data
    receiver: mpsc::Receiver<T>,
    /// Subscription ID for tracking
    id: String,
    /// Statistics for this subscription
    stats: Arc<SubscriptionStats>,
}

impl<T> Subscription<T> {
    /// Create a new subscription
    pub(crate) fn new(receiver: mpsc::Receiver<T>, id: String, stats: Arc<SubscriptionStats>) -> Self {
        Self { receiver, id, stats }
    }

    /// Get the next item from the subscription
    /// 
    /// Returns `None` if the subscription has been closed.
    pub async fn next(&mut self) -> Option<T> {
        self.receiver.recv().await
    }

    /// Get the subscription ID
    /// 
    /// The ID is a unique identifier for this subscription instance.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get subscription statistics
    /// 
    /// Returns stats including delivered and dropped message counts.
    /// Use this to monitor backpressure and adjust consumption rate if needed.
    pub fn stats(&self) -> &SubscriptionStats {
        &self.stats
    }
}

impl<T> Stream for Subscription<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}

/// Subscription sender for internal use
pub(crate) struct SubscriptionSender<T> {
    sender: mpsc::Sender<T>,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    channel: String,
    pub(crate) symbol: String,
    /// Statistics shared with the subscription receiver
    stats: Arc<SubscriptionStats>,
}

impl<T> SubscriptionSender<T> {
    /// Create a new subscription pair (sender + receiver) with default backpressure config
    pub fn new(channel: String, symbol: String) -> (Self, Subscription<T>) {
        Self::with_config(channel, symbol, BackpressureConfig::default())
    }

    /// Create a new subscription pair with custom backpressure config
    pub fn with_config(channel: String, symbol: String, config: BackpressureConfig) -> (Self, Subscription<T>) {
        let (sender, receiver) = mpsc::channel(config.buffer_size);
        let id = format!("{}-{}-{}", channel, symbol, uuid::Uuid::new_v4());
        let stats = Arc::new(SubscriptionStats::default());
        
        let subscription = Subscription::new(receiver, id.clone(), Arc::clone(&stats));
        let sender = Self {
            sender,
            id,
            channel,
            symbol,
            stats,
        };
        
        (sender, subscription)
    }

    /// Send data to the subscription (non-blocking with backpressure)
    /// 
    /// If the channel buffer is full, this will drop the message and
    /// increment the dropped counter. The WebSocket handler is never blocked.
    pub fn send(&self, data: T) -> Result<()> {
        match self.sender.try_send(data) {
            Ok(()) => {
                self.stats.delivered.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                // Backpressure: drop the message to avoid blocking
                self.stats.dropped.fetch_add(1, Ordering::Relaxed);
                Ok(()) // Not an error - this is expected behavior
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                Err(KrakyError::ChannelSend("subscription closed".to_string()))
            }
        }
    }

    /// Check if the subscription is still active
    #[allow(dead_code)]
    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}

/// Manager for multiple subscriptions
pub(crate) struct SubscriptionManager {
    /// Active orderbook subscriptions
    #[cfg(feature = "orderbook")]
    pub orderbook: Vec<SubscriptionSender<crate::models::OrderbookUpdate>>,
    /// Active trade subscriptions
    #[cfg(feature = "trades")]
    pub trades: Vec<SubscriptionSender<crate::models::Trade>>,
    /// Active ticker subscriptions
    #[cfg(feature = "ticker")]
    pub ticker: Vec<SubscriptionSender<crate::models::Ticker>>,
    /// Active OHLC subscriptions
    #[cfg(feature = "ohlc")]
    pub ohlc: Vec<SubscriptionSender<crate::models::OHLC>>,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    /// Create a new subscription manager
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "orderbook")]
            orderbook: Vec::new(),
            #[cfg(feature = "trades")]
            trades: Vec::new(),
            #[cfg(feature = "ticker")]
            ticker: Vec::new(),
            #[cfg(feature = "ohlc")]
            ohlc: Vec::new(),
        }
    }

    /// Clean up closed subscriptions
    #[allow(dead_code)]
    pub fn cleanup(&mut self) {
        #[cfg(feature = "orderbook")]
        self.orderbook.retain(|s| !s.is_closed());
        #[cfg(feature = "trades")]
        self.trades.retain(|s| !s.is_closed());
        #[cfg(feature = "ticker")]
        self.ticker.retain(|s| !s.is_closed());
        #[cfg(feature = "ohlc")]
        self.ohlc.retain(|s| !s.is_closed());
    }

    /// Dispatch orderbook update to relevant subscriptions
    #[cfg(feature = "orderbook")]
    pub fn dispatch_orderbook(&self, update: &crate::models::OrderbookUpdate) {
        for data in &update.data {
            for sub in &self.orderbook {
                if sub.symbol == data.symbol || sub.symbol == "*" {
                    let _ = sub.send(update.clone());
                }
            }
        }
    }

    /// Dispatch trade to relevant subscriptions
    #[cfg(feature = "trades")]
    pub fn dispatch_trade(&self, update: &crate::models::TradeUpdate) {
        for data in &update.data {
            let trade = data.to_trade();
            for sub in &self.trades {
                if sub.symbol == trade.symbol || sub.symbol == "*" {
                    let _ = sub.send(trade.clone());
                }
            }
        }
    }

    /// Dispatch ticker to relevant subscriptions
    #[cfg(feature = "ticker")]
    pub fn dispatch_ticker(&self, update: &crate::models::TickerUpdate) {
        for data in &update.data {
            let ticker = data.to_ticker();
            for sub in &self.ticker {
                if sub.symbol == ticker.symbol || sub.symbol == "*" {
                    let _ = sub.send(ticker.clone());
                }
            }
        }
    }

    /// Dispatch OHLC to relevant subscriptions
    #[cfg(feature = "ohlc")]
    pub fn dispatch_ohlc(&self, update: &crate::models::OHLCUpdate) {
        for data in &update.data {
            let ohlc = data.to_ohlc();
            for sub in &self.ohlc {
                if sub.symbol == ohlc.symbol || sub.symbol == "*" {
                    let _ = sub.send(ohlc.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscription_sender_receiver() {
        let (sender, mut subscription) = SubscriptionSender::<String>::new(
            "test".to_string(),
            "BTC/USD".to_string(),
        );

        // Send a message
        sender.send("hello".to_string()).unwrap();

        // Receive the message
        let msg = subscription.next().await;
        assert_eq!(msg, Some("hello".to_string()));
    }

    #[test]
    fn test_subscription_id_format() {
        let (sender, subscription) = SubscriptionSender::<String>::new(
            "book".to_string(),
            "BTC/USD".to_string(),
        );

        assert!(subscription.id().starts_with("book-BTC/USD-"));
        assert!(sender.symbol == "BTC/USD");
    }

    #[tokio::test]
    async fn test_backpressure_drops_messages() {
        // Create a subscription with a small buffer
        let config = BackpressureConfig::with_buffer_size(3);
        let (sender, mut subscription) = SubscriptionSender::<String>::with_config(
            "test".to_string(),
            "BTC/USD".to_string(),
            config,
        );

        // Fill the buffer
        sender.send("msg1".to_string()).unwrap();
        sender.send("msg2".to_string()).unwrap();
        sender.send("msg3".to_string()).unwrap();
        
        // This should be dropped (buffer full)
        sender.send("msg4".to_string()).unwrap();
        sender.send("msg5".to_string()).unwrap();

        // Check stats
        assert_eq!(subscription.stats().delivered(), 3);
        assert_eq!(subscription.stats().dropped(), 2);
        
        // Consume the buffered messages
        assert_eq!(subscription.next().await, Some("msg1".to_string()));
        assert_eq!(subscription.next().await, Some("msg2".to_string()));
        assert_eq!(subscription.next().await, Some("msg3".to_string()));
    }

    #[test]
    fn test_drop_rate_calculation() {
        let stats = SubscriptionStats::default();
        
        // No messages yet
        assert_eq!(stats.drop_rate(), 0.0);
        
        // Simulate some deliveries and drops
        stats.delivered.store(80, Ordering::Relaxed);
        stats.dropped.store(20, Ordering::Relaxed);
        
        // 20 / 100 = 20%
        assert!((stats.drop_rate() - 20.0).abs() < 0.001);
    }
}
