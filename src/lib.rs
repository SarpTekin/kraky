//! # Kraky
//!
//! A lightweight Rust SDK for connecting to Kraken's WebSocket API
//! and streaming market data.
//!
//! ## Features
//!
//! - Real-time orderbook updates with managed state
//! - Trade stream subscription
//! - Ticker updates
//! - OHLC (candlestick) data
//! - Automatic heartbeat handling
//! - Smart reconnection with exponential backoff
//! - Connection lifecycle event callbacks
//! - Clean async/await API
//! - **Backpressure control** with configurable buffer sizes
//! - **Modular feature flags** - opt-in to only what you need
//!
//! ## Feature Flags
//!
//! Kraky uses feature flags to allow you to compile only the functionality you need,
//! reducing binary size and compile times.
//!
//! ### Default Features
//!
//! By default, the following features are enabled:
//! - `reconnect` - Smart reconnection with exponential backoff
//! - `events` - Connection lifecycle event callbacks
//! - `orderbook` - Orderbook depth subscription and managed state
//!
//! ### Data Type Features
//!
//! Opt-in to additional data types you need:
//! - `trades` - Trade execution data subscription
//! - `ticker` - Ticker/quote data subscription
//! - `ohlc` - OHLC/candlestick data subscription
//!
//! ### Advanced Features
//!
//! - `analytics` - Orderbook imbalance analysis (requires `orderbook`)
//! - `checksum` - CRC32 orderbook validation (requires `orderbook`)
//! - `simd` - SIMD-accelerated JSON parsing (2-3x faster)
//!
//! ### Meta Features
//!
//! - `market-data` - Enables all data types: `orderbook`, `trades`, `ticker`, `ohlc`
//! - `full` - Enables all features including performance optimizations
//!
//! ### Usage Examples
//!
//! ```toml
//! # Default - orderbook with reconnection and events
//! kraky = "0.1"
//!
//! # Add trades support
//! kraky = { version = "0.1", features = ["trades"] }
//!
//! # All market data types
//! kraky = { version = "0.1", features = ["market-data"] }
//!
//! # Orderbook with analytics and checksum validation
//! kraky = { version = "0.1", features = ["analytics", "checksum"] }
//!
//! # Everything with SIMD performance
//! kraky = { version = "0.1", features = ["full", "simd"] }
//!
//! # Minimal - disable defaults, enable only what you need
//! kraky = { version = "0.1", default-features = false, features = ["orderbook", "reconnect"] }
//! ```
//!
//! ## Quick Start
//!
//! ```no_run
//! use kraky::KrakyClient;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Kraken WebSocket
//!     let client = KrakyClient::connect().await?;
//!
//!     // Subscribe to BTC/USD orderbook (requires 'orderbook' feature - enabled by default)
//!     let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//!     // Process updates
//!     while let Some(update) = orderbook.next().await {
//!         println!("Orderbook update for {}", update.data[0].symbol);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Examples
//!
//! ### Trades Stream (requires `trades` feature)
//!
//! Subscribe to real-time trade execution data with price, volume, and side information.
//!
//! ```no_run
//! # #[cfg(feature = "trades")]
//! # {
//! use kraky::KrakyClient;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut trades = client.subscribe_trades("BTC/USD").await?;
//!
//!     while let Some(trade) = trades.next().await {
//!         println!("{} - {} BTC @ ${} ({})",
//!             trade.timestamp,
//!             trade.qty,
//!             trade.price,
//!             if trade.side == kraky::TradeSide::Buy { "BUY" } else { "SELL" }
//!         );
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Ticker Updates (requires `ticker` feature)
//!
//! Subscribe to ticker data including best bid/ask, last price, and 24h volume.
//!
//! ```no_run
//! # #[cfg(feature = "ticker")]
//! # {
//! use kraky::KrakyClient;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut ticker = client.subscribe_ticker("BTC/USD").await?;
//!
//!     while let Some(tick) = ticker.next().await {
//!         println!("BTC/USD: ${} | Bid: ${} | Ask: ${} | 24h Vol: {} BTC",
//!             tick.last,
//!             tick.bid,
//!             tick.ask,
//!             tick.volume
//!         );
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### OHLC Candlesticks (requires `ohlc` feature)
//!
//! Subscribe to candlestick data with configurable time intervals (1m, 5m, 15m, 1h, 1d, etc.).
//!
//! ```no_run
//! # #[cfg(feature = "ohlc")]
//! # {
//! use kraky::{KrakyClient, Interval};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut ohlc = client.subscribe_ohlc("BTC/USD", Interval::Min5).await?;
//!
//!     while let Some(candle) = ohlc.next().await {
//!         println!("5m Candle - O: ${} H: ${} L: ${} C: ${} Vol: {}",
//!             candle.open,
//!             candle.high,
//!             candle.low,
//!             candle.close,
//!             candle.volume
//!         );
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Orderbook Analytics (requires `analytics` feature)
//!
//! Detect orderbook imbalances that indicate potential price movements. Unique to Kraky!
//!
//! ```no_run
//! # #[cfg(feature = "analytics")]
//! # {
//! use kraky::{KrakyClient, ImbalanceSignal};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//!     while let Some(_update) = orderbook.next().await {
//!         if let Some(ob) = client.get_orderbook("BTC/USD") {
//!             let metrics = ob.imbalance_metrics();
//!             let signal = metrics.signal(0.1); // 10% imbalance threshold
//!
//!             match signal {
//!                 ImbalanceSignal::Bullish => {
//!                     println!("🟢 BULLISH - Buy pressure: {:.2}%", metrics.imbalance_ratio * 100.0);
//!                 }
//!                 ImbalanceSignal::Bearish => {
//!                     println!("🔴 BEARISH - Sell pressure: {:.2}%", metrics.imbalance_ratio.abs() * 100.0);
//!                 }
//!                 ImbalanceSignal::Neutral => {
//!                     println!("⚪ NEUTRAL - Balanced orderbook");
//!                 }
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Checksum Validation (requires `checksum` feature)
//!
//! Validate orderbook data integrity using CRC32 checksums provided by Kraken.
//!
//! ```no_run
//! # #[cfg(feature = "checksum")]
//! # {
//! use kraky::KrakyClient;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//!     while let Some(update) = orderbook.next().await {
//!         if let Some(ob) = client.get_orderbook("BTC/USD") {
//!             let expected_checksum = update.data[0].checksum;
//!
//!             // Validate checksum manually
//!             if client.is_orderbook_valid("BTC/USD").unwrap_or(false) {
//!                 println!("✓ Checksum valid: {}", expected_checksum);
//!             } else {
//!                 println!("✗ Checksum mismatch for {}", update.data[0].symbol);
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### SIMD Performance (requires `simd` feature)
//!
//! Enable SIMD-accelerated JSON parsing for 2-3x faster message processing.
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! kraky = { version = "0.1", features = ["simd"] }
//! ```
//!
//! No code changes needed - just enable the feature flag for automatic performance boost!
//!
//! ### Reconnection Configuration (requires `reconnect` feature - enabled by default)
//!
//! Customize automatic reconnection behavior with exponential backoff.
//!
//! ```no_run
//! # #[cfg(feature = "reconnect")]
//! # {
//! use kraky::{KrakyClient, ReconnectConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ReconnectConfig {
//!         enabled: true,
//!         initial_delay: Duration::from_secs(1),
//!         max_delay: Duration::from_secs(60),
//!         backoff_multiplier: 2.0,
//!         max_attempts: Some(10),
//!     };
//!
//!     let client = KrakyClient::connect_with_config("wss://ws.kraken.com/v2", config).await?;
//!     // Client will automatically reconnect using your config
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### Connection Events (requires `events` feature - enabled by default)
//!
//! Monitor connection lifecycle with event callbacks for connected, disconnected, and reconnection states.
//!
//! ```no_run
//! # #[cfg(feature = "events")]
//! # {
//! use kraky::{KrakyClient, ConnectionEvent};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrakyClient::connect().await?;
//!     let mut events = client.subscribe_events();
//!
//!     while let Some(event) = events.recv().await {
//!         match event {
//!             ConnectionEvent::Connected => println!("✓ Connected to Kraken"),
//!             ConnectionEvent::Disconnected(reason) => {
//!                 println!("✗ Disconnected: {:?}", reason);
//!             }
//!             ConnectionEvent::Reconnecting(attempt) => {
//!                 println!("⟳ Reconnecting... (attempt {})", attempt);
//!             }
//!             ConnectionEvent::Reconnected => println!("✓ Reconnected to Kraken"),
//!             ConnectionEvent::ReconnectFailed(attempt, err) => {
//!                 println!("✗ Reconnect failed (attempt {}): {}", attempt, err);
//!             }
//!             ConnectionEvent::ReconnectExhausted => {
//!                 println!("✗ Reconnection attempts exhausted");
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ### WebSocket Trading (requires `trading` feature)
//!
//! Place and manage orders entirely via WebSocket - no REST API needed.
//!
//! ```no_run
//! # #[cfg(feature = "trading")]
//! # {
//! use kraky::{KrakyClient, Credentials, OrderParams, OrderSide};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let api_key = std::env::var("KRAKEN_API_KEY")?;
//!     let api_secret = std::env::var("KRAKEN_API_SECRET")?;
//!     let credentials = Credentials::new(api_key, api_secret);
//!
//!     let client = KrakyClient::connect().await?;
//!
//!     // Place a limit buy order for 0.001 BTC at $50,000
//!     let order = OrderParams {
//!         symbol: "BTC/USD".to_string(),
//!         side: OrderSide::Buy,
//!         order_type: kraky::OrderType::Limit,
//!         order_qty: Some(0.001),
//!         limit_price: Some(50000.0),
//!         trigger_price: None,
//!         time_in_force: None,
//!         post_only: None,
//!         reduce_only: None,
//!         stp: None,
//!         cl_ord_id: None,
//!         validate: None,
//!     };
//!
//!     let response = client.place_order(&credentials, order).await?;
//!     println!("Order placed! ID: {}", response.order_id);
//!
//!     // Cancel the order
//!     client.cancel_order(&credentials, &response.order_id).await?;
//!     println!("Order cancelled!");
//!
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ## Telegram Integration
//!
//! Build powerful alert bots that monitor markets 24/7 and send notifications to your phone.
//!
//! **Requires:** `telegram` or `telegram-alerts` feature
//!
//! ### Features
//!
//! - 🐋 **Whale Detection** - Alert on large orders (>10 BTC)
//! - 📊 **Imbalance Signals** - Bullish/Bearish orderbook signals
//! - 💰 **Price Alerts** - Threshold-based notifications
//! - 📈 **Spread Monitoring** - Unusual spread volatility
//! - 💼 **Account Activity** - Balance/order/execution updates (requires `private`)
//! - 🎯 **Trade Execution** - Order placement confirmations (requires `trading`)
//!
//! ### Setup
//!
//! 1. Create a Telegram bot with [@BotFather](https://t.me/botfather)
//! 2. Get your chat ID from [@userinfobot](https://t.me/userinfobot)
//! 3. Set environment variables:
//!
//! ```bash
//! export TELEGRAM_BOT_TOKEN="your_bot_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! ```
//!
//! ### Example: Imbalance Alert Bot
//!
//! ```no_run
//! # #[cfg(feature = "telegram-alerts")]
//! # {
//! use kraky::{KrakyClient, TelegramNotifier, ImbalanceSignal};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")?;
//!     let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")?.parse()?;
//!
//!     let client = KrakyClient::connect().await?;
//!     let bot = TelegramNotifier::new(&bot_token, chat_id);
//!
//!     client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//!     loop {
//!         if let Some(ob) = client.get_orderbook("BTC/USD") {
//!             let metrics = ob.imbalance_metrics();
//!             let signal = metrics.signal(0.1); // 10% threshold
//!
//!             if signal == ImbalanceSignal::Bullish {
//!                 bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
//!             } else if signal == ImbalanceSignal::Bearish {
//!                 bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
//!             }
//!         }
//!
//!         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
//!     }
//! }
//! # }
//! ```
//!
//! See `examples/telegram_imbalance_bot.rs`, `examples/whale_watcher.rs`, and other Telegram
//! examples in the repository.
//!
//! ## Authentication & Private Channels
//!
//! Access private WebSocket channels for account data using HMAC-SHA256 authentication.
//!
//! **Requires:** `auth` or `private` feature
//!
//! ### Setup Credentials
//!
//! 1. Log into [kraken.com](https://www.kraken.com)
//! 2. Settings → API → Create API Key
//! 3. Set permissions (view balances, view orders, etc.)
//! 4. Save the API Key and API Secret
//!
//! ```bash
//! export KRAKEN_API_KEY="your_api_key"
//! export KRAKEN_API_SECRET="your_base64_secret"
//! ```
//!
//! ### Example: Private Channels
//!
//! ```no_run
//! # #[cfg(feature = "private")]
//! # {
//! use kraky::{KrakyClient, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let api_key = std::env::var("KRAKEN_API_KEY")?;
//!     let api_secret = std::env::var("KRAKEN_API_SECRET")?;
//!
//!     let credentials = Credentials::new(api_key, api_secret);
//!     let nonce = std::time::SystemTime::now()
//!         .duration_since(std::time::UNIX_EPOCH)?
//!         .as_millis() as u64;
//!
//!     let token = credentials.generate_token(nonce)?;
//!     println!("Authentication token generated: {}...", &token[..20]);
//!
//!     Ok(())
//! }
//! # }
//! ```
//!
//! See `examples/auth_example.rs` and `examples/telegram_private_alerts.rs` for complete examples.
//!
//! ## WebSocket Trading
//!
//! Place, cancel, and manage orders entirely via WebSocket - no REST API needed.
//!
//! **Requires:** `trading` feature (automatically includes `auth` and `private`)
//!
//! ### Features
//!
//! - 📝 Place limit/market orders
//! - ❌ Cancel orders by ID or all orders
//! - 🔄 Amend existing orders
//! - 📊 Real-time order status updates
//! - 💥 Real-time execution notifications
//!
//! See `examples/telegram_trading_bot.rs` for a complete trading bot example.
//!
//! ## Feature Flag Architecture
//!
//! Kraky's features are organized in **layers**. Each layer builds on the previous one:
//!
//! ```text
//! Layer 4: INTEGRATIONS
//!   ├─ telegram-alerts (Telegram + analytics + ticker)
//!   └─ telegram (base Telegram integration)
//!
//! Layer 3: TRADING & PRIVATE DATA
//!   ├─ trading (place/cancel orders)
//!   ├─ private (balance/order/execution updates)
//!   └─ auth (HMAC-SHA256 signing)
//!
//! Layer 2: ANALYTICS & PERFORMANCE
//!   ├─ analytics (imbalance detection)
//!   ├─ checksum (CRC32 validation)
//!   └─ simd (SIMD JSON parsing)
//!
//! Layer 1: MARKET DATA TYPES
//!   ├─ orderbook (default)
//!   ├─ trades
//!   ├─ ticker
//!   └─ ohlc
//!
//! Layer 0: CORE (always included)
//!   ├─ reconnect
//!   ├─ events
//!   └─ orderbook
//! ```
//!
//! ## Binary Size Impact
//!
//! | Configuration | Size | vs Full | Added |
//! |--------------|------|---------|-------|
//! | full (everything) | 8.5 MB | baseline | — |
//! | market-data only | 7.8 MB | -8% | — |
//! | orderbook + trading | 7.25 MB | -15% | +53KB |
//! | orderbook + private | 7.23 MB | -15% | +50KB |
//! | orderbook (default) | 7.2 MB | -15% | — |
//! | trades only | 6.9 MB | -19% | — |
//!
//! **Key Takeaway:** Each feature adds minimal overhead:
//! - Authentication: ~50 KB (0.6%)
//! - Trading: ~3 KB (0.04%)
//! - Data types: 40-50 KB each
//!
//! ## Performance
//!
//! - **Async I/O** - Built on Tokio for efficient concurrent operations
//! - **Zero-copy parsing** - Serde JSON deserialization
//! - **Bounded channels** - Backpressure control prevents memory issues
//! - **BTreeMap orderbook** - O(log n) insertions/deletions with price ordering
//! - **Optional SIMD** - 2-3x faster JSON parsing with `simd` feature
//!
//! ## Examples
//!
//! The repository includes 18 working examples:
//!
//! **Basic (No Credentials):**
//! - `orderbook.rs` - Orderbook depth
//! - `trades.rs` - Trade stream
//! - `ticker.rs` - Price/volume
//! - `ohlc.rs` - Candlesticks
//! - `multi_subscribe.rs` - Multiple subscriptions
//! - `demo.rs` - Feature showcase
//!
//! **Advanced (Requires Setup):**
//! - `telegram_imbalance_bot.rs` - Imbalance alerts
//! - `whale_watcher.rs` - Large order detection
//! - `simple_price_alerts.rs` - Price thresholds
//! - `telegram_private_alerts.rs` - Account notifications
//! - `telegram_trading_bot.rs` - Automated trading
//! - `export_to_csv.rs` - Market data export
//!
//! See the `examples/` directory for all examples with detailed documentation.

pub mod client;
pub mod error;
pub mod messages;
pub mod models;
pub mod subscriptions;

// Authentication module (requires 'auth' feature)
#[cfg(feature = "auth")]
pub mod auth;

// Telegram bot integration (requires 'telegram' feature)
#[cfg(feature = "telegram")]
pub mod telegram;

// Re-export main types
pub use client::{ConnectionState, KrakyClient};

// Reconnection types (requires 'reconnect' feature)
#[cfg(feature = "reconnect")]
pub use client::ReconnectConfig;

// Connection event types (requires 'events' feature)
#[cfg(feature = "events")]
pub use client::ConnectionEvent;

// Error types (always available)
pub use error::{KrakenApiError, KrakenCategory, KrakenSeverity, KrakyError, Result};

// Data type exports (conditional on features)
#[cfg(feature = "orderbook")]
pub use models::{Orderbook, OrderbookSnapshot, OrderbookUpdate};

#[cfg(feature = "trades")]
pub use models::{Trade, TradeSide};

#[cfg(feature = "ticker")]
pub use models::Ticker;

#[cfg(feature = "ohlc")]
pub use models::{Interval, OHLC};

// Analytics types (requires both 'orderbook' and 'analytics' features)
#[cfg(all(feature = "orderbook", feature = "analytics"))]
pub use models::{ImbalanceMetrics, ImbalanceSignal};

// Checksum types (requires both 'orderbook' and 'checksum' features)
#[cfg(all(feature = "orderbook", feature = "checksum"))]
pub use models::ChecksumValidation;

// Private channel types (requires 'private' feature)
#[cfg(feature = "private")]
pub use models::{
    BalanceData, BalanceUpdate, ExecutionData, ExecutionUpdate, OrderData, OrderUpdate,
};

// Trading types (requires 'trading' feature)
#[cfg(feature = "trading")]
pub use models::{
    AmendOrderParams, AmendOrderResponse, CancelAllResponse, CancelOrderResponse, OrderParams,
    OrderResponse, OrderSide, OrderStatus, OrderType, SelfTradePrevention, TimeInForce,
};

// Subscription types (always available)
pub use subscriptions::{BackpressureConfig, Subscription, SubscriptionStats, DEFAULT_BUFFER_SIZE};

// Authentication types (requires 'auth' feature)
#[cfg(feature = "auth")]
pub use auth::Credentials;

// Telegram types (requires 'telegram' feature)
#[cfg(feature = "telegram")]
pub use telegram::TelegramNotifier;
