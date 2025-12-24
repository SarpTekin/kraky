//! Kraken WebSocket protocol messages

use serde::{Deserialize, Serialize};

/// Kraken WebSocket API v2 endpoint
pub const KRAKEN_WS_URL: &str = "wss://ws.kraken.com/v2";

/// Subscribe request method
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeRequest {
    /// Method name
    pub method: String,
    /// Request parameters
    pub params: SubscribeParams,
    /// Optional request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

/// Subscription parameters
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeParams {
    /// Channel name (book, trade, ticker, ohlc)
    pub channel: String,
    /// Trading pair symbols
    pub symbol: Vec<String>,
    /// Orderbook depth (for book channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    /// Snapshot requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// OHLC interval (for ohlc channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
}

impl SubscribeRequest {
    /// Create a new orderbook subscription request
    pub fn orderbook(symbols: Vec<String>, depth: u32) -> Self {
        Self {
            method: "subscribe".to_string(),
            params: SubscribeParams {
                channel: "book".to_string(),
                symbol: symbols,
                depth: Some(depth),
                snapshot: Some(true),
                interval: None,
            },
            req_id: None,
        }
    }

    /// Create a new trades subscription request
    pub fn trades(symbols: Vec<String>) -> Self {
        Self {
            method: "subscribe".to_string(),
            params: SubscribeParams {
                channel: "trade".to_string(),
                symbol: symbols,
                depth: None,
                snapshot: Some(true),
                interval: None,
            },
            req_id: None,
        }
    }

    /// Create a new ticker subscription request
    pub fn ticker(symbols: Vec<String>) -> Self {
        Self {
            method: "subscribe".to_string(),
            params: SubscribeParams {
                channel: "ticker".to_string(),
                symbol: symbols,
                depth: None,
                snapshot: Some(true),
                interval: None,
            },
            req_id: None,
        }
    }

    /// Create a new OHLC subscription request
    pub fn ohlc(symbols: Vec<String>, interval: u32) -> Self {
        Self {
            method: "subscribe".to_string(),
            params: SubscribeParams {
                channel: "ohlc".to_string(),
                symbol: symbols,
                depth: None,
                snapshot: Some(true),
                interval: Some(interval),
            },
            req_id: None,
        }
    }

    /// Set the request ID
    pub fn with_req_id(mut self, id: u64) -> Self {
        self.req_id = Some(id);
        self
    }
}

/// Unsubscribe request
#[derive(Debug, Clone, Serialize)]
pub struct UnsubscribeRequest {
    /// Method name
    pub method: String,
    /// Request parameters
    pub params: SubscribeParams,
    /// Optional request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

impl UnsubscribeRequest {
    /// Create a new unsubscribe request
    pub fn new(channel: String, symbols: Vec<String>) -> Self {
        Self {
            method: "unsubscribe".to_string(),
            params: SubscribeParams {
                channel,
                symbol: symbols,
                depth: None,
                snapshot: None,
                interval: None,
            },
            req_id: None,
        }
    }
}

/// Ping request for heartbeat
#[derive(Debug, Clone, Serialize)]
pub struct PingRequest {
    /// Method name
    pub method: String,
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

impl Default for PingRequest {
    fn default() -> Self {
        Self {
            method: "ping".to_string(),
            req_id: None,
        }
    }
}

/// Generic response from Kraken
#[derive(Debug, Clone, Deserialize)]
pub struct KrakenResponse {
    /// Channel name (for data messages)
    #[serde(default)]
    pub channel: String,
    /// Message type
    #[serde(rename = "type", default)]
    pub msg_type: String,
    /// Method (for responses)
    #[serde(default)]
    pub method: String,
    /// Success flag
    #[serde(default)]
    pub success: Option<bool>,
    /// Error message
    #[serde(default)]
    pub error: Option<String>,
    /// Request ID
    #[serde(default)]
    pub req_id: Option<u64>,
    /// Result data
    #[serde(default)]
    pub result: Option<serde_json::Value>,
}

/// System status message
#[derive(Debug, Clone, Deserialize)]
pub struct SystemStatus {
    /// Channel name
    pub channel: String,
    /// Message type
    #[serde(rename = "type")]
    pub msg_type: String,
    /// System data
    pub data: Vec<SystemStatusData>,
}

/// System status data
#[derive(Debug, Clone, Deserialize)]
pub struct SystemStatusData {
    /// API version
    pub api_version: String,
    /// Connection ID
    pub connection_id: u64,
    /// System status
    pub system: String,
    /// System version
    pub version: String,
}

/// Heartbeat message
#[derive(Debug, Clone, Deserialize)]
pub struct Heartbeat {
    /// Channel name
    pub channel: String,
    /// Message type
    #[serde(rename = "type")]
    pub msg_type: String,
}

/// Parsed message from Kraken WebSocket
#[derive(Debug, Clone)]
pub enum KrakyMessage {
    /// System status
    SystemStatus(SystemStatus),
    /// Heartbeat
    Heartbeat,
    /// Pong response
    Pong { req_id: Option<u64> },
    /// Subscription confirmation
    SubscriptionStatus {
        success: bool,
        channel: String,
        symbol: Option<String>,
        error: Option<String>,
    },
    /// Orderbook update
    #[cfg(feature = "orderbook")]
    Orderbook(crate::models::OrderbookUpdate),
    /// Trade update
    #[cfg(feature = "trades")]
    Trade(crate::models::TradeUpdate),
    /// Ticker update
    #[cfg(feature = "ticker")]
    Ticker(crate::models::TickerUpdate),
    /// OHLC update
    #[cfg(feature = "ohlc")]
    OHLC(crate::models::OHLCUpdate),
    /// Unknown message
    Unknown(serde_json::Value),
}

impl KrakyMessage {
    /// Parse a raw JSON message
    ///
    /// Uses SIMD-accelerated parsing when the `simd` feature is enabled.
    pub fn parse(text: &str) -> Result<Self, serde_json::Error> {
        // Parse JSON - use SIMD if feature is enabled
        #[cfg(feature = "simd")]
        let value: serde_json::Value = {
            let mut bytes = text.as_bytes().to_vec();
            simd_json::from_slice(&mut bytes).map_err(|e| {
                serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                ))
            })?
        };

        #[cfg(not(feature = "simd"))]
        let value: serde_json::Value = serde_json::from_str(text)?;

        // Check for method responses (pong, subscribe, unsubscribe)
        if let Some(method) = value.get("method").and_then(|m| m.as_str()) {
            match method {
                "pong" => {
                    let req_id = value.get("req_id").and_then(|r| r.as_u64());
                    return Ok(KrakyMessage::Pong { req_id });
                }
                "subscribe" | "unsubscribe" => {
                    let success = value
                        .get("success")
                        .and_then(|s| s.as_bool())
                        .unwrap_or(false);
                    let error = value
                        .get("error")
                        .and_then(|e| e.as_str())
                        .map(String::from);
                    let result = value.get("result");
                    let channel = result
                        .and_then(|r| r.get("channel"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();
                    let symbol = result
                        .and_then(|r| r.get("symbol"))
                        .and_then(|s| s.as_str())
                        .map(String::from);

                    return Ok(KrakyMessage::SubscriptionStatus {
                        success,
                        channel,
                        symbol,
                        error,
                    });
                }
                _ => {}
            }
        }

        // Check channel-based messages
        if let Some(channel) = value.get("channel").and_then(|c| c.as_str()) {
            match channel {
                "status" => {
                    let status: SystemStatus = serde_json::from_value(value)?;
                    return Ok(KrakyMessage::SystemStatus(status));
                }
                "heartbeat" => {
                    return Ok(KrakyMessage::Heartbeat);
                }
                #[cfg(feature = "orderbook")]
                "book" => {
                    let update: crate::models::OrderbookUpdate = serde_json::from_value(value)?;
                    return Ok(KrakyMessage::Orderbook(update));
                }
                #[cfg(feature = "trades")]
                "trade" => {
                    let update: crate::models::TradeUpdate = serde_json::from_value(value)?;
                    return Ok(KrakyMessage::Trade(update));
                }
                #[cfg(feature = "ticker")]
                "ticker" => {
                    let update: crate::models::TickerUpdate = serde_json::from_value(value)?;
                    return Ok(KrakyMessage::Ticker(update));
                }
                #[cfg(feature = "ohlc")]
                "ohlc" => {
                    let update: crate::models::OHLCUpdate = serde_json::from_value(value)?;
                    return Ok(KrakyMessage::OHLC(update));
                }
                _ => {}
            }
        }

        Ok(KrakyMessage::Unknown(value))
    }
}
