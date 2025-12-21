//! OHLC (candlestick) data types

use serde::{Deserialize, Serialize};

/// OHLC time interval
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Interval {
    /// 1 minute
    #[serde(rename = "1")]
    Min1 = 1,
    /// 5 minutes
    #[serde(rename = "5")]
    Min5 = 5,
    /// 15 minutes
    #[serde(rename = "15")]
    Min15 = 15,
    /// 30 minutes
    #[serde(rename = "30")]
    Min30 = 30,
    /// 1 hour
    #[serde(rename = "60")]
    Hour1 = 60,
    /// 4 hours
    #[serde(rename = "240")]
    Hour4 = 240,
    /// 1 day
    #[serde(rename = "1440")]
    Day1 = 1440,
    /// 1 week
    #[serde(rename = "10080")]
    Week1 = 10080,
    /// 15 days
    #[serde(rename = "21600")]
    Day15 = 21600,
}

impl Interval {
    /// Get the interval value in minutes
    pub fn minutes(&self) -> u32 {
        *self as u32
    }

    /// Convert to Kraken API string representation
    pub fn to_api_string(&self) -> String {
        self.minutes().to_string()
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Min1 => write!(f, "1m"),
            Interval::Min5 => write!(f, "5m"),
            Interval::Min15 => write!(f, "15m"),
            Interval::Min30 => write!(f, "30m"),
            Interval::Hour1 => write!(f, "1h"),
            Interval::Hour4 => write!(f, "4h"),
            Interval::Day1 => write!(f, "1d"),
            Interval::Week1 => write!(f, "1w"),
            Interval::Day15 => write!(f, "15d"),
        }
    }
}

/// OHLC candlestick data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLC {
    /// Trading pair symbol
    pub symbol: String,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume weighted average price
    pub vwap: f64,
    /// Volume
    pub volume: f64,
    /// Number of trades
    pub count: i64,
    /// Interval in minutes
    pub interval: u32,
    /// Candle start timestamp
    pub timestamp: String,
    /// Interval begin timestamp
    pub interval_begin: String,
}

/// Raw OHLC data from Kraken API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCDataRaw {
    /// Trading pair symbol
    pub symbol: String,
    /// Open price
    pub open: String,
    /// High price
    pub high: String,
    /// Low price
    pub low: String,
    /// Close price
    pub close: String,
    /// Volume weighted average price
    pub vwap: String,
    /// Volume
    pub volume: String,
    /// Number of trades
    pub count: i64,
    /// Interval in minutes
    pub interval: u32,
    /// Timestamp
    pub timestamp: String,
    /// Interval begin timestamp
    #[serde(default)]
    pub interval_begin: String,
}

impl OHLCDataRaw {
    /// Convert to typed OHLC
    pub fn to_ohlc(&self) -> OHLC {
        OHLC {
            symbol: self.symbol.clone(),
            open: self.open.parse().unwrap_or(0.0),
            high: self.high.parse().unwrap_or(0.0),
            low: self.low.parse().unwrap_or(0.0),
            close: self.close.parse().unwrap_or(0.0),
            vwap: self.vwap.parse().unwrap_or(0.0),
            volume: self.volume.parse().unwrap_or(0.0),
            count: self.count,
            interval: self.interval,
            timestamp: self.timestamp.clone(),
            interval_begin: self.interval_begin.clone(),
        }
    }
}

/// OHLC update message from Kraken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHLCUpdate {
    /// Channel name
    #[serde(default)]
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// OHLC data
    pub data: Vec<OHLCDataRaw>,
}

