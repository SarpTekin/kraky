//! Data models for Kraken WebSocket API.
//!
//! This module contains all data structures for Kraken WebSocket messages,
//! organized by feature flags for modular compilation.
//!
//! # Market Data Models
//!
//! ## Orderbook (default feature)
//!
//! Real-time orderbook depth with automatic state management:
//!
//! - [`Orderbook`] - Managed orderbook state with bid/ask levels
//! - [`OrderbookUpdate`] - Raw update messages from Kraken
//! - [`OrderbookSnapshot`] - Full orderbook snapshot
//!
//! Features:
//! - Automatic reconstruction from incremental updates
//! - Best bid/ask, spread, mid-price calculations
//! - Top N levels retrieval
//! - CRC32 checksum validation (with `checksum` feature)
//! - Imbalance detection (with `analytics` feature)
//!
//! ## Trades (requires `trades` feature)
//!
//! Real-time trade execution data:
//!
//! - [`Trade`] - Individual trade execution
//! - [`TradeSide`] - Buy or Sell side
//!
//! ## Ticker (requires `ticker` feature)
//!
//! Price and volume statistics:
//!
//! - [`Ticker`] - Ticker with bid/ask, last price, volume, 24h stats
//!
//! ## OHLC (requires `ohlc` feature)
//!
//! Candlestick data for technical analysis:
//!
//! - [`OHLC`] - Candlestick with open/high/low/close/volume
//! - [`Interval`] - Time intervals (1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w, 15d)
//!
//! # Private Channel Models (requires `private` feature)
//!
//! Account data structures for authenticated WebSocket channels:
//!
//! - [`BalanceUpdate`] - Real-time balance changes
//! - [`OrderUpdate`] - Order status updates
//! - [`ExecutionUpdate`] - Trade fill notifications
//!
//! # Trading Models (requires `trading` feature)
//!
//! Order management via WebSocket:
//!
//! - [`OrderParams`] - Order creation parameters
//! - [`AmendOrderParams`] - Order amendment parameters
//! - [`OrderResponse`] - Order placement response
//! - [`OrderType`] - Market, Limit order types
//! - [`OrderSide`] - Buy, Sell sides
//! - [`TimeInForce`] - GTC, IOC, GTD
//!
//! # Analytics Models (requires `analytics` feature)
//!
//! Advanced orderbook analysis:
//!
//! - [`ImbalanceMetrics`] - Bid/ask volume metrics
//! - [`ImbalanceSignal`] - Bullish, Bearish, Neutral signals
//!
//! # Example Usage
//!
//! ```no_run
//! # #[cfg(feature = "orderbook")]
//! # {
//! use kraky::KrakyClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KrakyClient::connect().await?;
//! let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
//!
//! while let Some(update) = orderbook.next().await {
//!     if let Some(ob) = client.get_orderbook("BTC/USD") {
//!         println!("Best bid: {:?}", ob.best_bid());
//!         println!("Best ask: {:?}", ob.best_ask());
//!         println!("Spread: {:?}", ob.spread());
//!
//!         #[cfg(feature = "analytics")]
//!         {
//!             let imbalance = ob.imbalance();
//!             println!("Imbalance: {:.2}%", imbalance * 100.0);
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! # }
//! ```

#[cfg(feature = "ohlc")]
mod ohlc;
#[cfg(feature = "orderbook")]
mod orderbook;
#[cfg(feature = "private")]
mod private;
#[cfg(feature = "ticker")]
mod ticker;
#[cfg(feature = "trades")]
mod trade;
#[cfg(feature = "trading")]
mod trading;

#[cfg(feature = "ohlc")]
pub use ohlc::*;
#[cfg(feature = "orderbook")]
pub use orderbook::*;
#[cfg(feature = "private")]
pub use private::*;
#[cfg(feature = "ticker")]
pub use ticker::*;
#[cfg(feature = "trades")]
pub use trade::*;
#[cfg(feature = "trading")]
pub use trading::*;
