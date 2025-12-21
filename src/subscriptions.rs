//! Subscription stream handling

use crate::error::{KrakyError, Result};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use futures_util::Stream;

/// A subscription to a Kraken data stream
/// 
/// Subscriptions are async streams that yield data as it arrives from
/// the Kraken WebSocket API. They can be consumed using the `next()` method
/// or by treating them as a `Stream`.
/// 
/// # Example
/// 
/// ```no_run
/// use kraky::KrakyClient;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = KrakyClient::connect().await?;
/// let mut trades = client.subscribe_trades("BTC/USD").await?;
/// 
/// while let Some(trade) = trades.next().await {
///     println!("Trade: {} @ {}", trade.qty, trade.price);
/// }
/// # Ok(())
/// # }
/// ```
pub struct Subscription<T> {
    /// Receiver for subscription data
    receiver: mpsc::UnboundedReceiver<T>,
    /// Subscription ID for tracking
    id: String,
}

impl<T> Subscription<T> {
    /// Create a new subscription
    pub(crate) fn new(receiver: mpsc::UnboundedReceiver<T>, id: String) -> Self {
        Self { receiver, id }
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
}

impl<T> Stream for Subscription<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_recv(cx)
    }
}

/// Subscription sender for internal use
pub(crate) struct SubscriptionSender<T> {
    sender: mpsc::UnboundedSender<T>,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    channel: String,
    pub(crate) symbol: String,
}

impl<T> SubscriptionSender<T> {
    /// Create a new subscription pair (sender + receiver)
    pub fn new(channel: String, symbol: String) -> (Self, Subscription<T>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let id = format!("{}-{}-{}", channel, symbol, uuid::Uuid::new_v4());
        
        let subscription = Subscription::new(receiver, id.clone());
        let sender = Self {
            sender,
            id,
            channel,
            symbol,
        };
        
        (sender, subscription)
    }

    /// Send data to the subscription
    pub fn send(&self, data: T) -> Result<()> {
        self.sender
            .send(data)
            .map_err(|e| KrakyError::ChannelSend(e.to_string()))
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
    pub orderbook: Vec<SubscriptionSender<crate::models::OrderbookUpdate>>,
    /// Active trade subscriptions
    pub trades: Vec<SubscriptionSender<crate::models::Trade>>,
    /// Active ticker subscriptions
    pub ticker: Vec<SubscriptionSender<crate::models::Ticker>>,
    /// Active OHLC subscriptions
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
            orderbook: Vec::new(),
            trades: Vec::new(),
            ticker: Vec::new(),
            ohlc: Vec::new(),
        }
    }

    /// Clean up closed subscriptions
    #[allow(dead_code)]
    pub fn cleanup(&mut self) {
        self.orderbook.retain(|s| !s.is_closed());
        self.trades.retain(|s| !s.is_closed());
        self.ticker.retain(|s| !s.is_closed());
        self.ohlc.retain(|s| !s.is_closed());
    }

    /// Dispatch orderbook update to relevant subscriptions
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
}
