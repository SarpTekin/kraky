//! Data models for Kraken WebSocket API

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
