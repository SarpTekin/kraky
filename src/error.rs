//! Error types for the Kraky

use thiserror::Error;

/// Result type alias for Kraky operations
pub type Result<T> = std::result::Result<T, KrakyError>;

/// Errors that can occur when using the Kraky
#[derive(Error, Debug)]
pub enum KrakyError {
    /// WebSocket connection error
    #[error("WebSocket connection error: {0}")]
    Connection(#[from] tokio_tungstenite::tungstenite::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing error
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// Channel send error
    #[error("Channel send error: {0}")]
    ChannelSend(String),

    /// Subscription error from Kraken API
    #[error("Subscription error: {0}")]
    Subscription(String),

    /// Invalid message received
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Connection closed unexpectedly
    #[error("Connection closed")]
    ConnectionClosed,

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimited,

    /// Invalid trading pair
    #[error("Invalid trading pair: {0}")]
    InvalidPair(String),
}

