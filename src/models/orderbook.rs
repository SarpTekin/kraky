//! Orderbook data types

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A price level in the orderbook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceLevel {
    /// Price at this level
    pub price: f64,
    /// Quantity at this level
    pub qty: f64,
    /// Timestamp of the update (Unix timestamp in seconds with decimals)
    #[serde(default)]
    pub timestamp: f64,
}

/// Orderbook update types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderbookUpdateType {
    /// Initial snapshot
    Snapshot,
    /// Incremental update
    Update,
}

impl std::fmt::Display for OrderbookUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderbookUpdateType::Snapshot => write!(f, "snapshot"),
            OrderbookUpdateType::Update => write!(f, "update"),
        }
    }
}

/// Raw orderbook update from Kraken WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookUpdate {
    /// Channel name
    #[serde(default)]
    pub channel: String,
    /// Update type (snapshot or update)
    #[serde(rename = "type")]
    pub update_type: OrderbookUpdateType,
    /// The orderbook data
    pub data: Vec<OrderbookData>,
}

/// Orderbook data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookData {
    /// Trading pair symbol
    pub symbol: String,
    /// Bid price levels (buy orders)
    #[serde(default)]
    pub bids: Vec<PriceLevelRaw>,
    /// Ask price levels (sell orders)
    #[serde(default)]
    pub asks: Vec<PriceLevelRaw>,
    /// Checksum for validation
    #[serde(default)]
    pub checksum: u32,
    /// Timestamp
    #[serde(default)]
    pub timestamp: String,
}

/// Raw price level from Kraken API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevelRaw {
    /// Price (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub price: f64,
    /// Quantity (can be number or string from API)
    #[serde(deserialize_with = "deserialize_number")]
    pub qty: f64,
}

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

impl PriceLevelRaw {
    /// Convert to typed PriceLevel
    pub fn to_price_level(&self) -> PriceLevel {
        PriceLevel {
            price: self.price,
            qty: self.qty,
            timestamp: 0.0,
        }
    }
}

/// Managed orderbook state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Orderbook {
    /// Trading pair symbol
    pub symbol: String,
    /// Bid levels (price -> quantity), sorted by price descending
    pub bids: BTreeMap<OrderedFloat, f64>,
    /// Ask levels (price -> quantity), sorted by price ascending
    pub asks: BTreeMap<OrderedFloat, f64>,
    /// Last update timestamp
    pub timestamp: String,
    /// Sequence number for ordering
    pub sequence: u64,
}

/// Wrapper for f64 that implements Ord for use in BTreeMap
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OrderedFloat(pub f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::hash::Hash for OrderedFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl Orderbook {
    /// Create a new empty orderbook
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            timestamp: String::new(),
            sequence: 0,
        }
    }

    /// Apply an update to the orderbook
    pub fn apply_update(&mut self, data: &OrderbookData) {
        self.timestamp = data.timestamp.clone();
        self.sequence += 1;

        // Apply bid updates
        for level in &data.bids {
            if level.qty == 0.0 {
                self.bids.remove(&OrderedFloat(level.price));
            } else {
                self.bids.insert(OrderedFloat(level.price), level.qty);
            }
        }

        // Apply ask updates
        for level in &data.asks {
            if level.qty == 0.0 {
                self.asks.remove(&OrderedFloat(level.price));
            } else {
                self.asks.insert(OrderedFloat(level.price), level.qty);
            }
        }
    }

    /// Get top N bid levels (highest prices first)
    pub fn top_bids(&self, n: usize) -> Vec<PriceLevel> {
        self.bids
            .iter()
            .rev()
            .take(n)
            .map(|(price, qty)| PriceLevel {
                price: price.0,
                qty: *qty,
                timestamp: 0.0,
            })
            .collect()
    }

    /// Get top N ask levels (lowest prices first)
    pub fn top_asks(&self, n: usize) -> Vec<PriceLevel> {
        self.asks
            .iter()
            .take(n)
            .map(|(price, qty)| PriceLevel {
                price: price.0,
                qty: *qty,
                timestamp: 0.0,
            })
            .collect()
    }

    /// Get the best bid price
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.keys().next_back().map(|p| p.0)
    }

    /// Get the best ask price
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.keys().next().map(|p| p.0)
    }

    /// Get the spread (best ask - best bid)
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get the mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }
}

/// Orderbook snapshot for time-travel feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookSnapshot {
    /// Unique snapshot ID
    pub id: String,
    /// Trading pair symbol
    pub symbol: String,
    /// Timestamp when snapshot was taken
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Bid levels
    pub bids: Vec<PriceLevel>,
    /// Ask levels
    pub asks: Vec<PriceLevel>,
    /// Sequence number
    pub sequence: u64,
}

impl OrderbookSnapshot {
    /// Create a snapshot from an orderbook
    pub fn from_orderbook(orderbook: &Orderbook, depth: usize) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            symbol: orderbook.symbol.clone(),
            timestamp: chrono::Utc::now(),
            bids: orderbook.top_bids(depth),
            asks: orderbook.top_asks(depth),
            sequence: orderbook.sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_new() {
        let ob = Orderbook::new("BTC/USD".to_string());
        assert_eq!(ob.symbol, "BTC/USD");
        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
        assert_eq!(ob.sequence, 0);
    }

    #[test]
    fn test_orderbook_apply_update() {
        let mut ob = Orderbook::new("BTC/USD".to_string());
        
        let update = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 1.5 },
                PriceLevelRaw { price: 49900.0, qty: 2.0 },
            ],
            asks: vec![
                PriceLevelRaw { price: 50100.0, qty: 1.0 },
                PriceLevelRaw { price: 50200.0, qty: 0.5 },
            ],
            checksum: 0,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };
        
        ob.apply_update(&update);
        
        assert_eq!(ob.bids.len(), 2);
        assert_eq!(ob.asks.len(), 2);
        assert_eq!(ob.sequence, 1);
    }

    #[test]
    fn test_orderbook_best_bid_ask() {
        let mut ob = Orderbook::new("BTC/USD".to_string());
        
        let update = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 1.5 },
                PriceLevelRaw { price: 49900.0, qty: 2.0 },
            ],
            asks: vec![
                PriceLevelRaw { price: 50100.0, qty: 1.0 },
                PriceLevelRaw { price: 50200.0, qty: 0.5 },
            ],
            checksum: 0,
            timestamp: "".to_string(),
        };
        
        ob.apply_update(&update);
        
        assert_eq!(ob.best_bid(), Some(50000.0));
        assert_eq!(ob.best_ask(), Some(50100.0));
    }

    #[test]
    fn test_orderbook_spread() {
        let mut ob = Orderbook::new("BTC/USD".to_string());
        
        let update = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 1.0 },
            ],
            asks: vec![
                PriceLevelRaw { price: 50100.0, qty: 1.0 },
            ],
            checksum: 0,
            timestamp: "".to_string(),
        };
        
        ob.apply_update(&update);
        
        assert_eq!(ob.spread(), Some(100.0));
        assert_eq!(ob.mid_price(), Some(50050.0));
    }

    #[test]
    fn test_orderbook_remove_level() {
        let mut ob = Orderbook::new("BTC/USD".to_string());
        
        // Add levels
        let update1 = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 1.0 },
            ],
            asks: vec![],
            checksum: 0,
            timestamp: "".to_string(),
        };
        ob.apply_update(&update1);
        assert_eq!(ob.bids.len(), 1);
        
        // Remove level (qty = 0)
        let update2 = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 0.0 },
            ],
            asks: vec![],
            checksum: 0,
            timestamp: "".to_string(),
        };
        ob.apply_update(&update2);
        assert_eq!(ob.bids.len(), 0);
    }

    #[test]
    fn test_top_bids_asks() {
        let mut ob = Orderbook::new("BTC/USD".to_string());
        
        let update = OrderbookData {
            symbol: "BTC/USD".to_string(),
            bids: vec![
                PriceLevelRaw { price: 50000.0, qty: 1.0 },
                PriceLevelRaw { price: 49900.0, qty: 2.0 },
                PriceLevelRaw { price: 49800.0, qty: 3.0 },
            ],
            asks: vec![
                PriceLevelRaw { price: 50100.0, qty: 1.0 },
                PriceLevelRaw { price: 50200.0, qty: 2.0 },
                PriceLevelRaw { price: 50300.0, qty: 3.0 },
            ],
            checksum: 0,
            timestamp: "".to_string(),
        };
        
        ob.apply_update(&update);
        
        let top_bids = ob.top_bids(2);
        assert_eq!(top_bids.len(), 2);
        assert_eq!(top_bids[0].price, 50000.0); // Highest bid first
        assert_eq!(top_bids[1].price, 49900.0);
        
        let top_asks = ob.top_asks(2);
        assert_eq!(top_asks.len(), 2);
        assert_eq!(top_asks[0].price, 50100.0); // Lowest ask first
        assert_eq!(top_asks[1].price, 50200.0);
    }

    #[test]
    fn test_ordered_float() {
        let a = OrderedFloat(1.5);
        let b = OrderedFloat(2.5);
        let c = OrderedFloat(1.5);
        
        assert!(a < b);
        assert_eq!(a, c);
        assert!(b > a);
    }

    #[test]
    fn test_price_level_raw_conversion() {
        let raw = PriceLevelRaw {
            price: 50000.50,
            qty: 1.25,
        };
        
        let level = raw.to_price_level();
        assert_eq!(level.price, 50000.50);
        assert_eq!(level.qty, 1.25);
    }
    
    #[test]
    fn test_deserialize_number_formats() {
        // Test deserializing from JSON with numbers
        let json = r#"{"price": 50000.0, "qty": 1.5}"#;
        let level: PriceLevelRaw = serde_json::from_str(json).unwrap();
        assert_eq!(level.price, 50000.0);
        assert_eq!(level.qty, 1.5);
        
        // Test deserializing from JSON with strings
        let json_str = r#"{"price": "49999.99", "qty": "2.5"}"#;
        let level_str: PriceLevelRaw = serde_json::from_str(json_str).unwrap();
        assert_eq!(level_str.price, 49999.99);
        assert_eq!(level_str.qty, 2.5);
    }
}

