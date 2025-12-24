//! Ticker data types

use serde::{Deserialize, Serialize};

/// Deserialize a value that could be either a number or a string representation of a number
fn deserialize_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct NumberVisitor;

    impl<'de> Visitor<'de> for NumberVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or string representation of a number")
        }

        fn visit_f64<E>(self, value: f64) -> Result<f64, E> {
            Ok(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<f64, E> {
            Ok(value as f64)
        }

        fn visit_u64<E>(self, value: u64) -> Result<f64, E> {
            Ok(value as f64)
        }

        fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where
            E: de::Error,
        {
            value.parse::<f64>().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(NumberVisitor)
}

/// Deserialize an optional value that could be either a number or a string representation of a number
fn deserialize_optional_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct OptionalNumberVisitor;

    impl<'de> Visitor<'de> for OptionalNumberVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number, string representation of a number, or null")
        }

        fn visit_none<E>(self) -> Result<Option<f64>, E> {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Option<f64>, E> {
            Ok(None)
        }

        fn visit_f64<E>(self, value: f64) -> Result<Option<f64>, E> {
            Ok(Some(value))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Option<f64>, E> {
            Ok(Some(value as f64))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Option<f64>, E> {
            Ok(Some(value as f64))
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<f64>, E>
        where
            E: de::Error,
        {
            value.parse::<f64>().map(Some).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(OptionalNumberVisitor)
}

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
    /// Best bid price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub bid: f64,
    /// Best bid quantity (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub bid_qty: f64,
    /// Best ask price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub ask: f64,
    /// Best ask quantity (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub ask_qty: f64,
    /// Last trade price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub last: f64,
    /// 24h volume (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub volume: f64,
    /// 24h volume weighted average price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub vwap: f64,
    /// 24h low price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub low: f64,
    /// 24h high price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub high: f64,
    /// 24h price change (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub change: f64,
    /// 24h price change percentage (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub change_pct: f64,
    /// 24h volume in USD (optional, can be number or string from API)
    #[serde(default, deserialize_with = "deserialize_optional_number")]
    pub volume_usd: Option<f64>,
    /// Timestamp
    #[serde(default)]
    pub timestamp: String,
}

impl TickerDataRaw {
    /// Convert to typed Ticker
    pub fn to_ticker(&self) -> Ticker {
        Ticker {
            symbol: self.symbol.clone(),
            bid: self.bid,
            bid_qty: self.bid_qty,
            ask: self.ask,
            ask_qty: self.ask_qty,
            last: self.last,
            volume: self.volume,
            vwap: self.vwap,
            low: self.low,
            high: self.high,
            change: self.change,
            change_pct: self.change_pct,
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
