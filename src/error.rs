//! Error types for the Kraky SDK
//! 
//! Provides structured error handling with Kraken-specific error parsing.

use thiserror::Error;
use std::fmt;

/// Result type alias for Kraky operations
pub type Result<T> = std::result::Result<T, KrakyError>;

/// Kraken API error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KrakenSeverity {
    /// Error - operation failed
    Error,
    /// Warning - operation succeeded but with issues
    Warning,
    /// Unknown severity
    Unknown,
}

impl fmt::Display for KrakenSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KrakenSeverity::Error => write!(f, "Error"),
            KrakenSeverity::Warning => write!(f, "Warning"),
            KrakenSeverity::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Kraken API error category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KrakenCategory {
    /// Query errors (invalid parameters, unknown pairs)
    Query,
    /// Service errors (unavailable, busy)
    Service,
    /// API errors (rate limits, invalid key)
    Api,
    /// Order errors (insufficient funds, invalid order)
    Order,
    /// Trade errors
    Trade,
    /// Funding errors
    Funding,
    /// Authentication errors
    Auth,
    /// General errors
    General,
    /// Unknown category
    Unknown(String),
}

impl fmt::Display for KrakenCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KrakenCategory::Query => write!(f, "Query"),
            KrakenCategory::Service => write!(f, "Service"),
            KrakenCategory::Api => write!(f, "API"),
            KrakenCategory::Order => write!(f, "Order"),
            KrakenCategory::Trade => write!(f, "Trade"),
            KrakenCategory::Funding => write!(f, "Funding"),
            KrakenCategory::Auth => write!(f, "Auth"),
            KrakenCategory::General => write!(f, "General"),
            KrakenCategory::Unknown(s) => write!(f, "{}", s),
        }
    }
}

/// Parsed Kraken API error
/// 
/// Kraken returns errors in the format: `"SeverityCategory:Message"`
/// For example: `"EQuery:Unknown asset pair"` or `"EService:Unavailable"`
/// 
/// This struct parses that format into structured fields for easier handling.
#[derive(Debug, Clone)]
pub struct KrakenApiError {
    /// Error severity (E = Error, W = Warning)
    pub severity: KrakenSeverity,
    /// Error category (Query, Service, API, etc.)
    pub category: KrakenCategory,
    /// Error message
    pub message: String,
    /// Original raw error string
    pub raw: String,
}

impl KrakenApiError {
    /// Parse a Kraken error string into structured error
    /// 
    /// # Examples
    /// 
    /// ```
    /// use kraky::KrakenApiError;
    /// 
    /// let err = KrakenApiError::parse("EQuery:Unknown asset pair");
    /// assert_eq!(err.message, "Unknown asset pair");
    /// ```
    pub fn parse(error: &str) -> Self {
        let raw = error.to_string();
        
        // Try to parse format: "ECategory:Message"
        if error.len() >= 2 {
            let severity = match error.chars().next() {
                Some('E') => KrakenSeverity::Error,
                Some('W') => KrakenSeverity::Warning,
                _ => KrakenSeverity::Unknown,
            };
            
            // Find the colon separator
            if let Some(colon_pos) = error.find(':') {
                let category_str = &error[1..colon_pos];
                let message = error[colon_pos + 1..].trim().to_string();
                
                let category = match category_str {
                    "Query" => KrakenCategory::Query,
                    "Service" => KrakenCategory::Service,
                    "API" => KrakenCategory::Api,
                    "Order" => KrakenCategory::Order,
                    "Trade" => KrakenCategory::Trade,
                    "Funding" => KrakenCategory::Funding,
                    "Auth" => KrakenCategory::Auth,
                    "General" => KrakenCategory::General,
                    other => KrakenCategory::Unknown(other.to_string()),
                };
                
                return Self {
                    severity,
                    category,
                    message,
                    raw,
                };
            }
        }
        
        // Fallback: couldn't parse, treat as general error
        Self {
            severity: KrakenSeverity::Error,
            category: KrakenCategory::General,
            message: error.to_string(),
            raw,
        }
    }
    
    /// Check if this is a temporary/retryable error
    pub fn is_retryable(&self) -> bool {
        matches!(self.category, KrakenCategory::Service)
            || self.message.contains("Unavailable")
            || self.message.contains("Busy")
            || self.message.contains("timeout")
    }
    
    /// Check if this is a rate limit error
    pub fn is_rate_limited(&self) -> bool {
        matches!(self.category, KrakenCategory::Api)
            && self.message.contains("Rate limit")
    }
    
    /// Check if this is an invalid pair error
    pub fn is_invalid_pair(&self) -> bool {
        matches!(self.category, KrakenCategory::Query)
            && (self.message.contains("Unknown asset pair")
                || self.message.contains("Invalid asset pair"))
    }
}

impl fmt::Display for KrakenApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}", self.severity, self.category, self.message)
    }
}

impl std::error::Error for KrakenApiError {}

/// Errors that can occur when using the Kraky SDK
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

    /// Kraken API error (parsed from API response)
    #[error("Kraken API error: {0}")]
    KrakenApi(KrakenApiError),

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

    /// Generic API error
    #[error("API error: {0}")]
    Api(String),
}

impl KrakyError {
    /// Create a KrakyError from a Kraken API error string
    /// 
    /// Parses the error and returns the appropriate error variant.
    pub fn from_kraken_error(error: &str) -> Self {
        let parsed = KrakenApiError::parse(error);
        
        // Map to specific error types for common cases
        if parsed.is_rate_limited() {
            KrakyError::RateLimited
        } else if parsed.is_invalid_pair() {
            KrakyError::InvalidPair(parsed.message.clone())
        } else {
            KrakyError::KrakenApi(parsed)
        }
    }
    
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            KrakyError::KrakenApi(e) => e.is_retryable(),
            KrakyError::Connection(_) => true,
            KrakyError::ConnectionClosed => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_error() {
        let err = KrakenApiError::parse("EQuery:Unknown asset pair");
        assert_eq!(err.severity, KrakenSeverity::Error);
        assert_eq!(err.category, KrakenCategory::Query);
        assert_eq!(err.message, "Unknown asset pair");
        assert!(err.is_invalid_pair());
    }

    #[test]
    fn test_parse_service_error() {
        let err = KrakenApiError::parse("EService:Unavailable");
        assert_eq!(err.severity, KrakenSeverity::Error);
        assert_eq!(err.category, KrakenCategory::Service);
        assert_eq!(err.message, "Unavailable");
        assert!(err.is_retryable());
    }

    #[test]
    fn test_parse_rate_limit_error() {
        let err = KrakenApiError::parse("EAPI:Rate limit exceeded");
        assert_eq!(err.severity, KrakenSeverity::Error);
        assert_eq!(err.category, KrakenCategory::Api);
        assert!(err.is_rate_limited());
    }

    #[test]
    fn test_parse_warning() {
        let err = KrakenApiError::parse("WQuery:Deprecated parameter");
        assert_eq!(err.severity, KrakenSeverity::Warning);
        assert_eq!(err.category, KrakenCategory::Query);
    }

    #[test]
    fn test_parse_unknown_format() {
        let err = KrakenApiError::parse("Some random error");
        assert_eq!(err.severity, KrakenSeverity::Error);
        assert_eq!(err.category, KrakenCategory::General);
        assert_eq!(err.message, "Some random error");
    }

    #[test]
    fn test_kraky_error_from_kraken() {
        // Rate limit should map to RateLimited
        let err = KrakyError::from_kraken_error("EAPI:Rate limit exceeded");
        assert!(matches!(err, KrakyError::RateLimited));

        // Invalid pair should map to InvalidPair
        let err = KrakyError::from_kraken_error("EQuery:Unknown asset pair");
        assert!(matches!(err, KrakyError::InvalidPair(_)));

        // Other errors should be KrakenApi
        let err = KrakyError::from_kraken_error("EService:Unavailable");
        assert!(matches!(err, KrakyError::KrakenApi(_)));
        assert!(err.is_retryable());
    }
}

