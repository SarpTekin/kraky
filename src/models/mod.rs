//! Data models for Kraken WebSocket API

#[cfg(feature = "orderbook")]
mod orderbook;
#[cfg(feature = "trades")]
mod trade;
#[cfg(feature = "ticker")]
mod ticker;
#[cfg(feature = "ohlc")]
mod ohlc;
#[cfg(feature = "private")]
mod private;
#[cfg(feature = "trading")]
mod trading;

#[cfg(feature = "orderbook")]
pub use orderbook::*;
#[cfg(feature = "trades")]
pub use trade::*;
#[cfg(feature = "ticker")]
pub use ticker::*;
#[cfg(feature = "ohlc")]
pub use ohlc::*;
#[cfg(feature = "private")]
pub use private::*;
#[cfg(feature = "trading")]
pub use trading::*;

