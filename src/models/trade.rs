//! Trade data types

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

/// Trade side (buy or sell)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    /// Buy order (taker bought)
    Buy,
    /// Sell order (taker sold)
    Sell,
}

impl std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeSide::Buy => write!(f, "buy"),
            TradeSide::Sell => write!(f, "sell"),
        }
    }
}

/// Order type for a trade (market or limit)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeOrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
}

/// A single trade event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trading pair symbol
    pub symbol: String,
    /// Trade side (buy/sell from taker's perspective)
    pub side: TradeSide,
    /// Trade price
    pub price: f64,
    /// Trade quantity
    pub qty: f64,
    /// Order type
    pub ord_type: TradeOrderType,
    /// Trade ID
    pub trade_id: i64,
    /// Timestamp
    pub timestamp: String,
}

/// Raw trade data from Kraken API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDataRaw {
    /// Trading pair symbol
    pub symbol: String,
    /// Trade side
    pub side: TradeSide,
    /// Price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub price: f64,
    /// Quantity (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub qty: f64,
    /// Order type
    pub ord_type: TradeOrderType,
    /// Trade ID
    pub trade_id: i64,
    /// Timestamp
    pub timestamp: String,
}

impl TradeDataRaw {
    /// Convert to typed Trade
    pub fn to_trade(&self) -> Trade {
        Trade {
            symbol: self.symbol.clone(),
            side: self.side,
            price: self.price,
            qty: self.qty,
            ord_type: self.ord_type,
            trade_id: self.trade_id,
            timestamp: self.timestamp.clone(),
        }
    }
}

/// Trade update message from Kraken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdate {
    /// Channel name
    #[serde(default)]
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// Trade data
    pub data: Vec<TradeDataRaw>,
}
