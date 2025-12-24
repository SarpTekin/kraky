//! Private WebSocket channel models
//!
//! Data types for authenticated WebSocket subscriptions including
//! balances, orders, and executions.
//!
//! Requires the `private` feature flag.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Balance update from the `balances` channel
///
/// Represents account balance changes for various assets.
///
/// # Example
/// ```no_run
/// # use kraky::{KrakyClient, Credentials};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let creds = Credentials::new("api_key", "api_secret");
/// let client = KrakyClient::with_credentials(creds).await?;
///
/// let mut balances = client.subscribe_balances().await?;
/// while let Some(update) = balances.next().await {
///     println!("BTC Balance: {:?}", update.balances.get("BTC"));
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdate {
    /// Channel name
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// Balance data
    pub data: Vec<BalanceData>,
}

/// Balance data for a single update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    /// Asset balances (symbol -> amount)
    #[serde(flatten)]
    pub balances: HashMap<String, String>,
}

/// Order update from the `orders` channel
///
/// Represents changes to your open orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdate {
    /// Channel name
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// Order data
    pub data: Vec<OrderData>,
}

/// Order data for a single order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderData {
    /// Order ID
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Trading pair
    pub symbol: String,
    /// Order side (buy/sell)
    pub side: String,
    /// Order type (limit/market)
    #[serde(rename = "order_type")]
    pub order_type: String,
    /// Limit price (if applicable)
    #[serde(rename = "limit_price")]
    pub limit_price: Option<String>,
    /// Order quantity
    #[serde(rename = "order_qty")]
    pub order_qty: String,
    /// Filled quantity
    #[serde(rename = "filled_qty", default)]
    pub filled_qty: String,
    /// Order status (pending/open/closed/cancelled)
    pub status: String,
    /// Timestamp
    #[serde(default)]
    pub timestamp: String,
}

/// Execution (trade fill) update from the `executions` channel
///
/// Represents when your orders are filled (fully or partially).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionUpdate {
    /// Channel name
    pub channel: String,
    /// Update type
    #[serde(rename = "type")]
    pub update_type: String,
    /// Execution data
    pub data: Vec<ExecutionData>,
}

/// Execution data for a single trade fill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionData {
    /// Execution ID
    #[serde(rename = "exec_id")]
    pub exec_id: String,
    /// Order ID that was filled
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Trading pair
    pub symbol: String,
    /// Side (buy/sell)
    pub side: String,
    /// Executed quantity
    #[serde(rename = "exec_qty")]
    pub exec_qty: String,
    /// Execution price
    #[serde(rename = "exec_price")]
    pub exec_price: String,
    /// Timestamp
    #[serde(default)]
    pub timestamp: String,
    /// Liquidity indicator (maker/taker)
    #[serde(default)]
    pub liquidity: String,
}

impl BalanceUpdate {
    /// Get balance for a specific asset
    pub fn get_balance(&self, asset: &str) -> Option<&String> {
        self.data.first()?.balances.get(asset)
    }

    /// Get all asset symbols in this update
    pub fn assets(&self) -> Vec<String> {
        self.data
            .first()
            .map(|d| d.balances.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl OrderUpdate {
    /// Check if this is an order open event
    pub fn is_open(&self) -> bool {
        self.update_type == "update" && self.data.iter().any(|o| o.status == "open")
    }

    /// Check if this is an order closed event
    pub fn is_closed(&self) -> bool {
        self.data.iter().any(|o| o.status == "closed" || o.status == "cancelled")
    }
}

impl ExecutionUpdate {
    /// Get total executed value for this update
    pub fn total_value(&self) -> Option<f64> {
        self.data.first().and_then(|e| {
            let qty: f64 = e.exec_qty.parse().ok()?;
            let price: f64 = e.exec_price.parse().ok()?;
            Some(qty * price)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_update_parsing() {
        let json = r#"{
            "channel": "balances",
            "type": "update",
            "data": [{
                "BTC": "1.5432",
                "USD": "50000.00"
            }]
        }"#;

        let update: BalanceUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.channel, "balances");
        assert_eq!(update.get_balance("BTC"), Some(&"1.5432".to_string()));
        assert_eq!(update.get_balance("USD"), Some(&"50000.00".to_string()));
    }

    #[test]
    fn test_order_update_parsing() {
        let json = r#"{
            "channel": "orders",
            "type": "update",
            "data": [{
                "order_id": "O12345",
                "symbol": "BTC/USD",
                "side": "buy",
                "order_type": "limit",
                "limit_price": "95000.00",
                "order_qty": "0.5",
                "filled_qty": "0.0",
                "status": "open",
                "timestamp": "2024-01-01T00:00:00Z"
            }]
        }"#;

        let update: OrderUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.channel, "orders");
        assert!(update.is_open());
        assert!(!update.is_closed());
    }

    #[test]
    fn test_execution_update_parsing() {
        let json = r#"{
            "channel": "executions",
            "type": "update",
            "data": [{
                "exec_id": "E12345",
                "order_id": "O12345",
                "symbol": "BTC/USD",
                "side": "buy",
                "exec_qty": "0.5",
                "exec_price": "95000.00",
                "timestamp": "2024-01-01T00:00:00Z",
                "liquidity": "taker"
            }]
        }"#;

        let update: ExecutionUpdate = serde_json::from_str(json).unwrap();
        assert_eq!(update.channel, "executions");
        assert_eq!(update.total_value(), Some(47500.0));
    }
}
