//! Ticker data types

use serde::{Deserialize, Serialize};

/// Ticker information for a trading pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Trading pair symbol
    pub symbol: String,
    /// Best bid price
    pub bid: f64,
    /// Best bid quantity
    pub bid_qty: f64,
    /// Best ask price
    pub ask: f64,
    /// Best ask quantity
    pub ask_qty: f64,
    /// Last trade price
    pub last: f64,
    /// 24h volume
    pub volume: f64,
    /// 24h volume weighted average price
    pub vwap: f64,
    /// 24h low price
    pub low: f64,
    /// 24h high price
    pub high: f64,
    /// 24h price change
    pub change: f64,
    /// 24h price change percentage
    pub change_pct: f64,
}

/// Raw ticker data from Kraken API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerDataRaw {
    /// Trading pair symbol
    pub symbol: String,
    /// Best bid price
    pub bid: String,
    /// Best bid quantity
    pub bid_qty: String,
    /// Best ask price
    pub ask: String,
    /// Best ask quantity
    pub ask_qty: String,
    /// Last trade price
    pub last: String,
    /// 24h volume
    pub volume: String,
    /// 24h volume weighted average price
    pub vwap: String,
    /// 24h low price
    pub low: String,
    /// 24h high price
    pub high: String,
    /// 24h price change
    pub change: String,
    /// 24h price change percentage
    pub change_pct: String,
}

impl TickerDataRaw {
    /// Convert to typed Ticker
    pub fn to_ticker(&self) -> Ticker {
        Ticker {
            symbol: self.symbol.clone(),
            bid: self.bid.parse().unwrap_or(0.0),
            bid_qty: self.bid_qty.parse().unwrap_or(0.0),
            ask: self.ask.parse().unwrap_or(0.0),
            ask_qty: self.ask_qty.parse().unwrap_or(0.0),
            last: self.last.parse().unwrap_or(0.0),
            volume: self.volume.parse().unwrap_or(0.0),
            vwap: self.vwap.parse().unwrap_or(0.0),
            low: self.low.parse().unwrap_or(0.0),
            high: self.high.parse().unwrap_or(0.0),
            change: self.change.parse().unwrap_or(0.0),
            change_pct: self.change_pct.parse().unwrap_or(0.0),
        }
    }
}

/// Ticker update message from Kraken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerUpdate {
    /// Channel name
    #[serde(default)]
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// Ticker data
    pub data: Vec<TickerDataRaw>,
}

