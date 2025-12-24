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
