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
//! - Clean async/await API
//! 
//! ## Quick Start
//! 
//! ```no_run
//! use kraky::{KrakyClient, Interval};
//! use futures_util::StreamExt;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Kraken WebSocket
//!     let client = KrakyClient::connect().await?;
//!     
//!     // Subscribe to BTC/USD orderbook
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
//! ### Orderbook
//! 
//! Subscribe to orderbook depth updates with configurable depth levels
//! (10, 25, 100, 500, or 1000 levels).
//! 
//! ### Trades
//! 
//! Subscribe to real-time trade execution data.
//! 
//! ### Ticker
//! 
//! Subscribe to ticker updates including bid/ask, volume, and price changes.
//! 
//! ### OHLC
//! 
//! Subscribe to candlestick data with configurable intervals.

pub mod client;
pub mod error;
pub mod messages;
pub mod models;
pub mod subscriptions;

// Re-export main types
pub use client::KrakyClient;
pub use error::{KrakyError, Result};
pub use models::{
    Interval, Orderbook, OrderbookSnapshot, OrderbookUpdate,
    OHLC, Ticker, Trade, TradeSide,
};
pub use subscriptions::Subscription;

