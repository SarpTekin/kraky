//! Trade data types

use serde::{Deserialize, Serialize};

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

/// Order type for a trade
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
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
    pub ord_type: OrderType,
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
    /// Price as string
    pub price: String,
    /// Quantity as string
    pub qty: String,
    /// Order type
    pub ord_type: OrderType,
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
            price: self.price.parse().unwrap_or(0.0),
            qty: self.qty.parse().unwrap_or(0.0),
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

