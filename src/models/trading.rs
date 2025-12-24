//! Trading models for Kraken WebSocket API v2
//!
//! Order placement, cancellation, and management via WebSocket.
//! Requires the `trading` feature flag.

use serde::{Deserialize, Serialize};

/// Order side (buy or sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    /// Market order - executes immediately at best available price
    Market,
    /// Limit order - executes at specified price or better
    Limit,
    /// Stop-loss market order
    StopLoss,
    /// Stop-loss limit order
    StopLossLimit,
    /// Take-profit market order
    TakeProfit,
    /// Take-profit limit order
    TakeProfitLimit,
    /// Trailing stop market order
    TrailingStop,
    /// Trailing stop limit order
    TrailingStopLimit,
    /// Iceberg order (hidden volume)
    Iceberg,
}

/// Time-in-force options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    /// Good-til-cancelled (default)
    GTC,
    /// Immediate-or-cancel
    IOC,
    /// Fill-or-kill
    FOK,
    /// Good-til-date
    GTD,
}

/// Self-trade prevention mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SelfTradePrevention {
    /// Cancel the newest order
    CancelNewest,
    /// Cancel the oldest order
    CancelOldest,
    /// Cancel both orders
    CancelBoth,
}

/// Parameters for placing an order
#[derive(Debug, Clone, Serialize)]
pub struct OrderParams {
    /// Trading pair (e.g., "BTC/USD")
    pub symbol: String,
    /// Order side (buy or sell)
    pub side: OrderSide,
    /// Order type
    pub order_type: OrderType,
    /// Order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    /// Limit price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    /// Trigger price (for stop orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Post-only (order will only be placed if it would be a maker order)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Reduce-only (order can only reduce an existing position)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<SelfTradePrevention>,
    /// Client order ID (optional, for tracking)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Validate only (dry-run without executing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

impl OrderParams {
    /// Create a market buy order
    pub fn market_buy(symbol: impl Into<String>, quantity: f64) -> Self {
        Self {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            order_qty: Some(quantity),
            limit_price: None,
            trigger_price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
            stp: None,
            cl_ord_id: None,
            validate: None,
        }
    }

    /// Create a market sell order
    pub fn market_sell(symbol: impl Into<String>, quantity: f64) -> Self {
        Self {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            order_qty: Some(quantity),
            limit_price: None,
            trigger_price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
            stp: None,
            cl_ord_id: None,
            validate: None,
        }
    }

    /// Create a limit buy order
    pub fn limit_buy(symbol: impl Into<String>, quantity: f64, price: f64) -> Self {
        Self {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            order_qty: Some(quantity),
            limit_price: Some(price),
            trigger_price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
            stp: None,
            cl_ord_id: None,
            validate: None,
        }
    }

    /// Create a limit sell order
    pub fn limit_sell(symbol: impl Into<String>, quantity: f64, price: f64) -> Self {
        Self {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            order_qty: Some(quantity),
            limit_price: Some(price),
            trigger_price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
            stp: None,
            cl_ord_id: None,
            validate: None,
        }
    }

    /// Set time-in-force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set post-only flag
    pub fn with_post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Set client order ID
    pub fn with_client_id(mut self, id: impl Into<String>) -> Self {
        self.cl_ord_id = Some(id.into());
        self
    }

    /// Set validate flag (dry-run mode)
    pub fn with_validate(mut self, validate: bool) -> Self {
        self.validate = Some(validate);
        self
    }

    /// Set self-trade prevention mode
    pub fn with_stp(mut self, stp: SelfTradePrevention) -> Self {
        self.stp = Some(stp);
        self
    }
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
    Triggered,
}

/// Response from placing an order
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResponse {
    /// Order ID assigned by Kraken
    pub order_id: String,
    /// Client order ID (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order status
    pub order_status: OrderStatus,
    /// Timestamp
    pub timestamp: String,
}

/// Parameters for amending an order
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderParams {
    /// Order ID to amend
    pub order_id: String,
    /// New quantity (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    /// New limit price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    /// New trigger price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<f64>,
}

/// Response from amending an order
#[derive(Debug, Clone, Deserialize)]
pub struct AmendOrderResponse {
    /// Order ID
    pub order_id: String,
    /// Amendment was successful
    pub success: bool,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response from canceling an order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// Order ID that was cancelled
    pub order_id: String,
    /// Cancellation was successful
    pub success: bool,
}

/// Response from cancel all orders
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllResponse {
    /// Number of orders cancelled
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_buy_order() {
        let order = OrderParams::market_buy("BTC/USD", 0.1);
        assert_eq!(order.symbol, "BTC/USD");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.order_qty, Some(0.1));
    }

    #[test]
    fn test_limit_sell_order() {
        let order = OrderParams::limit_sell("ETH/USD", 1.0, 2500.0);
        assert_eq!(order.symbol, "ETH/USD");
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.order_qty, Some(1.0));
        assert_eq!(order.limit_price, Some(2500.0));
    }

    #[test]
    fn test_order_builder() {
        let order = OrderParams::limit_buy("BTC/USD", 0.5, 50000.0)
            .with_time_in_force(TimeInForce::IOC)
            .with_post_only(true)
            .with_client_id("my-order-123");

        assert_eq!(order.time_in_force, Some(TimeInForce::IOC));
        assert_eq!(order.post_only, Some(true));
        assert_eq!(order.cl_ord_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_validate_mode() {
        let order = OrderParams::market_buy("BTC/USD", 0.1)
            .with_validate(true);

        assert_eq!(order.validate, Some(true));
    }
}
