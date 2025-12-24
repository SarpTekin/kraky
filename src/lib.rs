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
//! ## Subscription Types
//!
//! ### Orderbook (enabled by default)
//!
//! Subscribe to orderbook depth updates with configurable depth levels
//! (10, 25, 100, 500, or 1000 levels).
//!
//! ### Trades (requires `trades` feature)
//!
//! Subscribe to real-time trade execution data.
//!
//! ### Ticker (requires `ticker` feature)
//!
//! Subscribe to ticker updates including bid/ask, volume, and price changes.
//!
//! ### OHLC (requires `ohlc` feature)
//!
//! Subscribe to candlestick data with configurable intervals.
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
//!             let signal = ob.imbalance_signal();
//!
//!             if signal == ImbalanceSignal::Bullish {
//!                 bot.send_message("🟢 BULLISH signal on BTC/USD!").await?;
//!             } else if signal == ImbalanceSignal::Bearish {
//!                 bot.send_message("🔴 BEARISH signal on BTC/USD!").await?;
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
//! The repository includes 19 working examples:
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
