//! Telegram notification integration for Kraky SDK
//!
//! This module provides real-time market alerts via Telegram, leveraging
//! Kraky's advanced orderbook analytics.
//!
//! ## Features
//! - Price alerts (above/below thresholds)
//! - Orderbook imbalance signals (bullish/bearish/neutral)
//! - Customizable alert formatting
//! - Async/await compatible
//!
//! ## Quick Start
//!
//! ```no_run
//! use kraky::telegram::TelegramNotifier;
//! use kraky::{KrakyClient, ImbalanceSignal};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let bot = TelegramNotifier::new("YOUR_BOT_TOKEN", 123456789);
//!
//!     let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//!     while let Some(_) = orderbook.next().await {
//!         if let Some(ob) = client.get_orderbook("BTC/USD") {
//!             let metrics = ob.imbalance_metrics();
//!             let signal = metrics.signal(0.15);
//!
//!             if !matches!(signal, ImbalanceSignal::Neutral) {
//!                 bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use teloxide::prelude::*;
use crate::error::{KrakyError, Result};

#[cfg(feature = "analytics")]
use crate::models::{ImbalanceMetrics, ImbalanceSignal};

/// Telegram notification client for real-time market alerts
///
/// Provides methods to send formatted alerts to a Telegram chat,
/// including price updates and orderbook imbalance signals.
pub struct TelegramNotifier {
    bot: Bot,
    chat_id: ChatId,
}

impl TelegramNotifier {
    /// Create a new Telegram notifier
    ///
    /// # Arguments
    /// * `token` - Telegram bot token from @BotFather
    /// * `chat_id` - Telegram chat ID to send messages to
    ///
    /// # Example
    /// ```no_run
    /// use kraky::telegram::TelegramNotifier;
    ///
    /// let bot = TelegramNotifier::new("123456:ABC-DEF", 987654321);
    /// ```
    pub fn new(token: &str, chat_id: i64) -> Self {
        Self {
            bot: Bot::new(token),
            chat_id: ChatId(chat_id),
        }
    }

    /// Send a basic text alert
    ///
    /// # Arguments
    /// * `message` - The message to send
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_alert("BTC/USD reached $100,000!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_alert(&self, message: &str) -> Result<()> {
        self.bot
            .send_message(self.chat_id, message)
            .await
            .map_err(|e| KrakyError::InvalidMessage(format!("Telegram error: {}", e)))?;
        Ok(())
    }

    /// Send a price alert with formatting
    ///
    /// # Arguments
    /// * `symbol` - Trading pair (e.g., "BTC/USD")
    /// * `price` - Current price
    /// * `context` - Additional context (e.g., "above threshold")
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_price_alert("BTC/USD", 100000.0, "Target reached!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_price_alert(
        &self,
        symbol: &str,
        price: f64,
        context: &str,
    ) -> Result<()> {
        let message = format!(
            "💰 {} Price Alert\n\
            Price: ${:.2}\n\
            {}",
            symbol, price, context
        );
        self.send_alert(&message).await
    }

    /// Send an orderbook imbalance alert (requires 'analytics' feature)
    ///
    /// This showcases Kraky's unique orderbook analytics capabilities by
    /// sending detailed imbalance metrics and trading signals.
    ///
    /// # Arguments
    /// * `symbol` - Trading pair (e.g., "BTC/USD")
    /// * `metrics` - Imbalance metrics from orderbook
    /// * `signal` - Trading signal (Bullish/Bearish/Neutral)
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # use kraky::{KrakyClient, ImbalanceSignal};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// let client = KrakyClient::connect().await?;
    ///
    /// if let Some(ob) = client.get_orderbook("BTC/USD") {
    ///     let metrics = ob.imbalance_metrics();
    ///     let signal = metrics.signal(0.15);
    ///     bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "analytics")]
    pub async fn send_imbalance_alert(
        &self,
        symbol: &str,
        metrics: &ImbalanceMetrics,
        signal: ImbalanceSignal,
    ) -> Result<()> {
        let (emoji, signal_name, description) = match signal {
            ImbalanceSignal::Bullish => (
                "🟢",
                "BULLISH",
                "Strong buy pressure detected - more bids than asks"
            ),
            ImbalanceSignal::Bearish => (
                "🔴",
                "BEARISH",
                "Strong sell pressure detected - more asks than bids"
            ),
            ImbalanceSignal::Neutral => (
                "⚪",
                "NEUTRAL",
                "Balanced orderbook - no clear directional bias"
            ),
        };

        let message = format!(
            "{} {} Orderbook Imbalance Alert\n\
            \n\
            📊 Signal: {}\n\
            {}\n\
            \n\
            📈 Metrics:\n\
            • Bid Volume: {:.4} BTC\n\
            • Ask Volume: {:.4} BTC\n\
            • Bid/Ask Ratio: {:.2}\n\
            • Imbalance: {:+.2}%\n\
            \n\
            💡 Interpretation:\n\
            {}",
            emoji,
            symbol,
            signal_name,
            "─".repeat(30),
            metrics.bid_volume,
            metrics.ask_volume,
            metrics.bid_ask_ratio,
            metrics.imbalance_ratio * 100.0,
            description
        );

        self.send_alert(&message).await
    }

    /// Send a threshold-based price alert
    ///
    /// # Arguments
    /// * `symbol` - Trading pair
    /// * `price` - Current price
    /// * `threshold` - Threshold price
    /// * `above` - True if price is above threshold, false if below
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_threshold_alert("BTC/USD", 100500.0, 100000.0, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_threshold_alert(
        &self,
        symbol: &str,
        price: f64,
        threshold: f64,
        above: bool,
    ) -> Result<()> {
        let (emoji, direction) = if above {
            ("📈", "above")
        } else {
            ("📉", "below")
        };

        let change_pct = ((price - threshold) / threshold * 100.0).abs();

        let message = format!(
            "{} {} Threshold Alert\n\
            \n\
            Current Price: ${:.2}\n\
            Threshold: ${:.2}\n\
            Status: Price is {} threshold\n\
            Change: {:.2}%",
            emoji, symbol, price, threshold, direction, change_pct
        );

        self.send_alert(&message).await
    }

    /// Send a formatted orderbook snapshot summary
    ///
    /// # Arguments
    /// * `symbol` - Trading pair
    /// * `best_bid` - Best bid price
    /// * `best_ask` - Best ask price
    /// * `spread` - Bid-ask spread
    /// * `mid_price` - Mid price
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_orderbook_summary("BTC/USD", 99500.0, 99505.0, 5.0, 99502.5).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_orderbook_summary(
        &self,
        symbol: &str,
        best_bid: f64,
        best_ask: f64,
        spread: f64,
        mid_price: f64,
    ) -> Result<()> {
        let spread_bps = (spread / mid_price) * 10000.0;

        let message = format!(
            "📖 {} Orderbook Update\n\
            \n\
            Best Bid: ${:.2}\n\
            Best Ask: ${:.2}\n\
            Mid Price: ${:.2}\n\
            Spread: ${:.2} ({:.1} bps)",
            symbol, best_bid, best_ask, mid_price, spread, spread_bps
        );

        self.send_alert(&message).await
    }

    /// Send a connection status update
    ///
    /// # Arguments
    /// * `connected` - Whether the client is connected
    /// * `details` - Additional details about the connection
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_connection_status(true, "Connected to Kraken WebSocket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_connection_status(&self, connected: bool, details: &str) -> Result<()> {
        let (emoji, status) = if connected {
            ("✅", "Connected")
        } else {
            ("❌", "Disconnected")
        };

        let message = format!(
            "{} Connection Status: {}\n\
            {}",
            emoji, status, details
        );

        self.send_alert(&message).await
    }

    /// Send a whale alert for large orders
    ///
    /// Detects and reports significant order placements in the orderbook,
    /// helping traders identify when large players ("whales") are active.
    ///
    /// # Arguments
    /// * `symbol` - Trading pair (e.g., "BTC/USD")
    /// * `side` - Order side ("bid" or "ask")
    /// * `price` - Price level of the large order
    /// * `volume` - Size of the order
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_whale_alert("BTC/USD", "bid", 95000.0, 50.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_whale_alert(
        &self,
        symbol: &str,
        side: &str,
        price: f64,
        volume: f64,
    ) -> Result<()> {
        let (emoji, direction) = if side.to_lowercase() == "bid" {
            ("🟢", "BUY")
        } else {
            ("🔴", "SELL")
        };

        let message = format!(
            "🐋 {} Whale Alert!\n\
            \n\
            {} Large {} Order Detected\n\
            {}\n\
            \n\
            Price: ${:.2}\n\
            Volume: {:.4} {}\n\
            Total Value: ${:.2}\n\
            \n\
            💡 A large {} order has appeared in the orderbook.\n\
            This could indicate institutional activity.",
            symbol,
            emoji,
            direction,
            "─".repeat(30),
            price,
            volume,
            symbol.split('/').next().unwrap_or(""),
            price * volume,
            side.to_lowercase()
        );

        self.send_alert(&message).await
    }

    /// Send a spread volatility alert
    ///
    /// Alerts when the bid-ask spread widens significantly beyond normal levels,
    /// which often indicates decreasing liquidity or upcoming volatility.
    ///
    /// # Arguments
    /// * `symbol` - Trading pair
    /// * `current_spread_bps` - Current spread in basis points
    /// * `normal_spread_bps` - Normal/average spread in basis points
    /// * `multiplier` - How many times wider than normal (e.g., 3.5x)
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_spread_alert("BTC/USD", 15.0, 5.0, 3.0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_spread_alert(
        &self,
        symbol: &str,
        current_spread_bps: f64,
        normal_spread_bps: f64,
        multiplier: f64,
    ) -> Result<()> {
        let severity = if multiplier >= 5.0 {
            ("🚨", "CRITICAL")
        } else if multiplier >= 3.0 {
            ("⚠️", "HIGH")
        } else {
            ("⚡", "MODERATE")
        };

        let message = format!(
            "{} {} Spread Volatility Alert\n\
            \n\
            Severity: {}\n\
            {}\n\
            \n\
            Current Spread: {:.1} bps\n\
            Normal Spread: {:.1} bps\n\
            Multiplier: {:.1}x normal\n\
            \n\
            💡 Interpretation:\n\
            The bid-ask spread has widened significantly, indicating\n\
            reduced liquidity. This often precedes increased volatility\n\
            or large price movements.",
            severity.0,
            symbol,
            severity.1,
            "─".repeat(30),
            current_spread_bps,
            normal_spread_bps,
            multiplier
        );

        self.send_alert(&message).await
    }

    /// Send an order flow divergence alert
    ///
    /// Detects when price action diverges from orderbook pressure, which can
    /// signal potential reversals or unusual market dynamics.
    ///
    /// # Arguments
    /// * `symbol` - Trading pair
    /// * `price_change` - Recent price change percentage
    /// * `orderbook_signal` - Current orderbook imbalance signal
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # use kraky::ImbalanceSignal;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_divergence_alert("BTC/USD", 2.5, ImbalanceSignal::Bearish).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "analytics")]
    pub async fn send_divergence_alert(
        &self,
        symbol: &str,
        price_change: f64,
        orderbook_signal: ImbalanceSignal,
    ) -> Result<()> {
        let price_direction = if price_change > 0.0 { "UP" } else { "DOWN" };
        let price_emoji = if price_change > 0.0 { "📈" } else { "📉" };

        let (ob_emoji, ob_signal) = match orderbook_signal {
            ImbalanceSignal::Bullish => ("🟢", "BULLISH"),
            ImbalanceSignal::Bearish => ("🔴", "BEARISH"),
            ImbalanceSignal::Neutral => ("⚪", "NEUTRAL"),
        };

        // Determine if this is a divergence
        let is_divergence = (price_change > 0.0 && matches!(orderbook_signal, ImbalanceSignal::Bearish))
            || (price_change < 0.0 && matches!(orderbook_signal, ImbalanceSignal::Bullish));

        if !is_divergence {
            return Ok(()); // Only send alerts on actual divergence
        }

        let message = format!(
            "⚡ {} Order Flow DIVERGENCE Alert\n\
            \n\
            🎯 Divergence Detected!\n\
            {}\n\
            \n\
            {} Price Action: {} ({:+.2}%)\n\
            {} Orderbook: {}\n\
            \n\
            💡 Interpretation:\n\
            Price is moving {} but orderbook shows {} pressure.\n\
            This divergence could indicate:\n\
            • Potential trend reversal\n\
            • Large hidden orders executing\n\
            • Market maker positioning\n\
            \n\
            ⚠️ Exercise caution - divergences often precede volatility.",
            symbol,
            "─".repeat(30),
            price_emoji,
            price_direction,
            price_change,
            ob_emoji,
            ob_signal,
            price_direction,
            ob_signal
        );

        self.send_alert(&message).await
    }

    /// Send a trade execution alert
    ///
    /// Reports when significant trades execute, helping track market activity
    /// and large player movements.
    ///
    /// # Arguments
    /// * `symbol` - Trading pair
    /// * `side` - Trade side ("buy" or "sell")
    /// * `price` - Execution price
    /// * `volume` - Trade volume
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::telegram::TelegramNotifier;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// bot.send_trade_alert("BTC/USD", "buy", 96500.0, 25.5).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_trade_alert(
        &self,
        symbol: &str,
        side: &str,
        price: f64,
        volume: f64,
    ) -> Result<()> {
        let (emoji, direction) = if side.to_lowercase() == "buy" {
            ("🟢", "BUY")
        } else {
            ("🔴", "SELL")
        };

        let total_value = price * volume;

        let message = format!(
            "💥 {} Large Trade Executed\n\
            \n\
            {} {} Order Filled\n\
            {}\n\
            \n\
            Price: ${:.2}\n\
            Volume: {:.4} {}\n\
            Total Value: ${:.2}\n\
            \n\
            💡 A significant {} trade just executed.\n\
            This represents real market activity.",
            symbol,
            emoji,
            direction,
            "─".repeat(30),
            price,
            volume,
            symbol.split('/').next().unwrap_or(""),
            total_value,
            side.to_lowercase()
        );

        self.send_alert(&message).await
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PRIVATE WEBSOCKET NOTIFICATIONS (requires 'private' feature)
    // ═══════════════════════════════════════════════════════════════════════

    /// Send a balance update notification
    ///
    /// Alerts when your account balance changes.
    /// Requires both `telegram` and `private` features.
    ///
    /// # Arguments
    /// * `balance_update` - Balance update data from private WebSocket
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::{TelegramNotifier, BalanceUpdate};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// // Assuming you received a balance update from WebSocket
    /// # let balance_update = serde_json::from_str::<BalanceUpdate>(r#"{"channel":"balances","type":"update","data":[{"BTC":"1.5","USD":"50000"}]}"#)?;
    /// bot.send_balance_update(&balance_update).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "private")]
    pub async fn send_balance_update(
        &self,
        update: &crate::models::BalanceUpdate,
    ) -> Result<()> {
        if let Some(data) = update.data.first() {
            let mut balance_lines = Vec::new();

            for (asset, amount) in &data.balances {
                balance_lines.push(format!("  {} {}", amount, asset));
            }

            let message = format!(
                "💰 Balance Update\n\
                \n\
                {}\n\
                {}\n\
                \n\
                🕐 {}\n\
                \n\
                Your account balances have been updated.",
                "─".repeat(30),
                balance_lines.join("\n"),
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );

            self.send_alert(&message).await
        } else {
            Ok(())
        }
    }

    /// Send an order update notification
    ///
    /// Alerts when your order status changes (opened, filled, cancelled).
    /// Requires both `telegram` and `private` features.
    ///
    /// # Arguments
    /// * `order_update` - Order update data from private WebSocket
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::{TelegramNotifier, OrderUpdate};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// # let order_update = serde_json::from_str::<OrderUpdate>(r#"{"channel":"orders","type":"update","data":[{"order_id":"O123","symbol":"BTC/USD","side":"buy","order_type":"limit","limit_price":"95000","order_qty":"0.5","filled_qty":"0","status":"open","timestamp":"2024-01-01T00:00:00Z"}]}"#)?;
    /// bot.send_order_update(&order_update).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "private")]
    pub async fn send_order_update(
        &self,
        update: &crate::models::OrderUpdate,
    ) -> Result<()> {
        if let Some(order) = update.data.first() {
            let emoji = match order.status.as_str() {
                "open" => "🟢",
                "closed" => "✅",
                "cancelled" => "❌",
                "pending" => "⏳",
                _ => "📋",
            };

            let status_text = match order.status.as_str() {
                "open" => "OPENED".to_string(),
                "closed" => "FILLED".to_string(),
                "cancelled" => "CANCELLED".to_string(),
                "pending" => "PENDING".to_string(),
                _ => order.status.to_uppercase(),
            };

            let side_emoji = if order.side.to_lowercase() == "buy" { "🟢" } else { "🔴" };

            let mut details = vec![
                format!("{} {} Order", side_emoji, order.side.to_uppercase()),
                format!("Order ID: {}", order.order_id),
                format!("Type: {}", order.order_type),
            ];

            if let Some(limit_price) = &order.limit_price {
                details.push(format!("Limit Price: ${}", limit_price));
            }

            details.push(format!("Quantity: {}", order.order_qty));

            if !order.filled_qty.is_empty() && order.filled_qty != "0" && order.filled_qty != "0.0" {
                details.push(format!("Filled: {}", order.filled_qty));
            }

            let message = format!(
                "{} {} Order {}\n\
                \n\
                📊 {}\n\
                {}\n\
                \n\
                {}\n\
                \n\
                🕐 {}",
                emoji,
                order.symbol,
                status_text,
                "─".repeat(30),
                details.join("\n"),
                "─".repeat(30),
                if order.timestamp.is_empty() {
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()
                } else {
                    order.timestamp.clone()
                }
            );

            self.send_alert(&message).await
        } else {
            Ok(())
        }
    }

    /// Send an execution (trade fill) alert
    ///
    /// Alerts when your order is executed (filled).
    /// Requires both `telegram` and `private` features.
    ///
    /// # Arguments
    /// * `execution_update` - Execution update data from private WebSocket
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::{TelegramNotifier, ExecutionUpdate};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// # let execution = serde_json::from_str::<ExecutionUpdate>(r#"{"channel":"executions","type":"update","data":[{"exec_id":"E123","order_id":"O123","symbol":"BTC/USD","side":"buy","exec_qty":"0.5","exec_price":"95000","timestamp":"2024-01-01T00:00:00Z","liquidity":"taker"}]}"#)?;
    /// bot.send_execution_alert(&execution).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "private")]
    pub async fn send_execution_alert(
        &self,
        update: &crate::models::ExecutionUpdate,
    ) -> Result<()> {
        if let Some(exec) = update.data.first() {
            let (side_emoji, side_text) = if exec.side.to_lowercase() == "buy" {
                ("🟢", "BOUGHT")
            } else {
                ("🔴", "SOLD")
            };

            let qty: f64 = exec.exec_qty.parse().unwrap_or(0.0);
            let price: f64 = exec.exec_price.parse().unwrap_or(0.0);
            let total_value = qty * price;

            let asset = exec.symbol.split('/').next().unwrap_or("BTC");

            let liquidity_emoji = if exec.liquidity.to_lowercase() == "maker" {
                "🏭" // Maker (provided liquidity)
            } else {
                "⚡" // Taker (removed liquidity)
            };

            let message = format!(
                "💥 {} Trade Executed!\n\
                \n\
                {} {} {} {}\n\
                {}\n\
                \n\
                Execution ID: {}\n\
                Order ID: {}\n\
                \n\
                Price: ${}\n\
                Quantity: {} {}\n\
                Total Value: ${:.2}\n\
                \n\
                {} Liquidity: {}\n\
                \n\
                🕐 {}",
                exec.symbol,
                side_emoji,
                side_text,
                exec.exec_qty,
                asset,
                "─".repeat(30),
                exec.exec_id,
                exec.order_id,
                exec.exec_price,
                exec.exec_qty,
                asset,
                total_value,
                liquidity_emoji,
                exec.liquidity.to_uppercase(),
                if exec.timestamp.is_empty() {
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()
                } else {
                    exec.timestamp.clone()
                }
            );

            self.send_alert(&message).await
        } else {
            Ok(())
        }
    }

    /// Send a combined portfolio summary
    ///
    /// Sends a formatted summary of all balances.
    /// Useful for periodic portfolio updates.
    /// Requires both `telegram` and `private` features.
    ///
    /// # Arguments
    /// * `balance_update` - Balance update data
    ///
    /// # Example
    /// ```no_run
    /// # use kraky::{TelegramNotifier, BalanceUpdate};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bot = TelegramNotifier::new("token", 123);
    /// # let balance_update = serde_json::from_str::<BalanceUpdate>(r#"{"channel":"balances","type":"update","data":[{"BTC":"1.5","ETH":"10.0","USD":"50000"}]}"#)?;
    /// bot.send_portfolio_summary(&balance_update).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "private")]
    pub async fn send_portfolio_summary(
        &self,
        update: &crate::models::BalanceUpdate,
    ) -> Result<()> {
        if let Some(data) = update.data.first() {
            let mut crypto_balances = Vec::new();
            let mut fiat_balances = Vec::new();

            for (asset, amount) in &data.balances {
                let line = format!("  {} {}", amount, asset);

                // Separate crypto from fiat
                if asset == "USD" || asset == "EUR" || asset == "GBP" {
                    fiat_balances.push(line);
                } else {
                    crypto_balances.push(line);
                }
            }

            let mut message = format!(
                "📊 Portfolio Summary\n\
                {}\n\
                \n",
                "═".repeat(30)
            );

            if !crypto_balances.is_empty() {
                message.push_str("💎 Crypto Assets:\n");
                message.push_str(&crypto_balances.join("\n"));
                message.push_str("\n\n");
            }

            if !fiat_balances.is_empty() {
                message.push_str("💵 Fiat Balances:\n");
                message.push_str(&fiat_balances.join("\n"));
                message.push_str("\n\n");
            }

            message.push_str(&format!(
                "{}\n\
                🕐 {}\n\
                \n\
                Total Assets: {}",
                "═".repeat(30),
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
                data.balances.len()
            ));

            self.send_alert(&message).await
        } else {
            Ok(())
        }
    }

    // ============================================================================
    // Trading Notifications (requires 'trading' feature)
    // ============================================================================

    /// Send order placement notification
    ///
    /// Alerts when an order has been successfully placed.
    #[cfg(feature = "trading")]
    pub async fn send_order_placed(
        &self,
        response: &crate::models::OrderResponse,
        params: &crate::models::OrderParams,
    ) -> Result<()> {
        let side_emoji = match params.side {
            crate::models::OrderSide::Buy => "🟢",
            crate::models::OrderSide::Sell => "🔴",
        };

        let order_type = format!("{:?}", params.order_type);

        let message = format!(
            "{} Order Placed\n\
            {}\n\
            \n\
            Order ID: {}\n\
            Symbol: {}\n\
            Side: {} {:?}\n\
            Type: {}\n\
            Quantity: {}\n\
            {}\n\
            Status: {:?}\n\
            \n\
            {} Order successfully submitted to exchange",
            side_emoji,
            "═".repeat(35),
            response.order_id,
            params.symbol,
            side_emoji,
            params.side,
            order_type,
            params.order_qty.map(|q| format!("{:.6}", q)).unwrap_or("N/A".to_string()),
            match params.limit_price {
                Some(price) => format!("Limit Price: ${:.2}", price),
                None => "Market Price".to_string(),
            },
            response.order_status,
            if params.validate.unwrap_or(false) { "✓" } else { "💸" }
        );

        self.send_alert(&message).await
    }

    /// Send order filled notification
    ///
    /// Alerts when an order has been fully or partially filled.
    #[cfg(feature = "trading")]
    pub async fn send_order_filled(
        &self,
        symbol: &str,
        side: &crate::models::OrderSide,
        quantity: f64,
        price: f64,
        order_id: &str,
    ) -> Result<()> {
        let side_emoji = match side {
            crate::models::OrderSide::Buy => "🟢",
            crate::models::OrderSide::Sell => "🔴",
        };

        let total_value = quantity * price;

        let message = format!(
            "✅ Order Filled\n\
            {}\n\
            \n\
            Symbol: {}\n\
            Side: {} {:?}\n\
            Filled: {:.6}\n\
            Price: ${:.2}\n\
            Total: ${:.2}\n\
            \n\
            Order ID: {}\n\
            \n\
            💰 Trade executed successfully",
            "═".repeat(35),
            symbol,
            side_emoji,
            side,
            quantity,
            price,
            total_value,
            order_id
        );

        self.send_alert(&message).await
    }

    /// Send order cancelled notification
    ///
    /// Alerts when an order has been cancelled.
    #[cfg(feature = "trading")]
    pub async fn send_order_cancelled(
        &self,
        symbol: &str,
        order_id: &str,
        reason: Option<&str>,
    ) -> Result<()> {
        let message = format!(
            "🚫 Order Cancelled\n\
            {}\n\
            \n\
            Symbol: {}\n\
            Order ID: {}\n\
            {}\n\
            \n\
            ℹ️  Order removed from orderbook",
            "═".repeat(35),
            symbol,
            order_id,
            reason.map(|r| format!("Reason: {}", r)).unwrap_or_default()
        );

        self.send_alert(&message).await
    }

    /// Send order failed notification
    ///
    /// Alerts when an order placement has failed.
    #[cfg(feature = "trading")]
    pub async fn send_order_failed(
        &self,
        params: &crate::models::OrderParams,
        error: &str,
    ) -> Result<()> {
        let message = format!(
            "❌ Order Failed\n\
            {}\n\
            \n\
            Symbol: {}\n\
            Side: {:?}\n\
            Type: {:?}\n\
            \n\
            Error: {}\n\
            \n\
            ⚠️  Please check order parameters and try again",
            "═".repeat(35),
            params.symbol,
            params.side,
            params.order_type,
            error
        );

        self.send_alert(&message).await
    }

    /// Send order amended notification
    ///
    /// Alerts when an order has been successfully modified.
    #[cfg(feature = "trading")]
    pub async fn send_order_amended(
        &self,
        response: &crate::models::AmendOrderResponse,
        params: &crate::models::AmendOrderParams,
    ) -> Result<()> {
        let mut changes = Vec::new();

        if let Some(qty) = params.order_qty {
            changes.push(format!("Quantity: {:.6}", qty));
        }
        if let Some(price) = params.limit_price {
            changes.push(format!("Limit Price: ${:.2}", price));
        }
        if let Some(trigger) = params.trigger_price {
            changes.push(format!("Trigger Price: ${:.2}", trigger));
        }

        let message = format!(
            "📝 Order Amended\n\
            {}\n\
            \n\
            Order ID: {}\n\
            \n\
            Changes:\n\
            {}\n\
            \n\
            {} Order successfully modified",
            "═".repeat(35),
            response.order_id,
            changes.join("\n"),
            if response.success { "✅" } else { "❌" }
        );

        self.send_alert(&message).await
    }

    /// Send daily trading summary
    ///
    /// Provides a summary of trading activity.
    #[cfg(feature = "trading")]
    pub async fn send_trading_summary(
        &self,
        total_trades: usize,
        total_volume: f64,
        profit_loss: f64,
        win_rate: f64,
    ) -> Result<()> {
        let pl_emoji = if profit_loss >= 0.0 { "📈" } else { "📉" };
        let pl_sign = if profit_loss >= 0.0 { "+" } else { "" };

        let message = format!(
            "📊 Daily Trading Summary\n\
            {}\n\
            {}\n\
            \n\
            Total Trades: {}\n\
            Total Volume: ${:.2}\n\
            \n\
            {} P&L: {}{:.2}\n\
            Win Rate: {:.1}%\n\
            \n\
            {} End of day report",
            "═".repeat(35),
            chrono::Utc::now().format("%Y-%m-%d"),
            total_trades,
            total_volume,
            pl_emoji,
            pl_sign,
            profit_loss,
            win_rate,
            "📋"
        );

        self.send_alert(&message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifier_creation() {
        let notifier = TelegramNotifier::new("test_token", 12345);
        assert_eq!(notifier.chat_id, ChatId(12345));
    }

    #[cfg(feature = "analytics")]
    #[test]
    fn test_signal_formatting() {
        // Test that signal types are properly handled
        let signals = vec![
            ImbalanceSignal::Bullish,
            ImbalanceSignal::Bearish,
            ImbalanceSignal::Neutral,
        ];

        for signal in signals {
            // Just verify the match doesn't panic
            let _ = match signal {
                ImbalanceSignal::Bullish => "🟢",
                ImbalanceSignal::Bearish => "🔴",
                ImbalanceSignal::Neutral => "⚪",
            };
        }
    }
}
