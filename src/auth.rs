//! Authentication module for Kraken WebSocket API
//!
//! This module provides HMAC-SHA256 signing for authenticated WebSocket subscriptions.
//!
//! Requires the `auth` feature flag.

use crate::error::{KrakyError, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Authentication credentials for Kraken API
#[derive(Clone)]
pub struct Credentials {
    /// API key (public)
    pub api_key: String,
    /// API secret (private, base64 encoded)
    api_secret: String,
}

impl Credentials {
    /// Create new credentials
    ///
    /// # Arguments
    /// * `api_key` - Your Kraken API key
    /// * `api_secret` - Your Kraken API secret (base64 encoded string from Kraken)
    ///
    /// # Example
    /// ```no_run
    /// use kraky::Credentials;
    ///
    /// let creds = Credentials::new(
    ///     "your_api_key_here",
    ///     "your_base64_secret_here"
    /// );
    /// ```
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }

    /// Generate authentication token for WebSocket subscription
    ///
    /// Uses HMAC-SHA256 to sign the request according to Kraken API specs.
    ///
    /// # Arguments
    /// * `nonce` - Unique nonce (timestamp in nanoseconds recommended)
    ///
    /// # Returns
    /// Base64-encoded HMAC signature
    pub fn generate_token(&self, nonce: u64) -> Result<String> {
        // Decode the API secret from base64
        let secret_bytes = BASE64
            .decode(&self.api_secret)
            .map_err(|e| KrakyError::InvalidMessage(format!("Invalid API secret: {}", e)))?;

        // Create the message to sign: nonce as string
        let message = nonce.to_string();

        // Create HMAC-SHA256 with the decoded secret
        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .map_err(|e| KrakyError::InvalidMessage(format!("HMAC error: {}", e)))?;

        // Sign the message
        mac.update(message.as_bytes());

        // Get the signature
        let signature = mac.finalize().into_bytes();

        // Encode to base64
        Ok(BASE64.encode(signature))
    }

    /// Get API key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_creation() {
        let creds = Credentials::new("test_key", "dGVzdF9zZWNyZXQ=");
        assert_eq!(creds.api_key(), "test_key");
    }

    #[test]
    fn test_token_generation() {
        // Test with a valid base64 secret
        let creds = Credentials::new("test_key", "dGVzdF9zZWNyZXQ=");
        let token = creds.generate_token(1234567890);
        assert!(token.is_ok());
        assert!(!token.unwrap().is_empty());
    }

    #[test]
    fn test_invalid_secret() {
        // Test with invalid base64
        let creds = Credentials::new("test_key", "not-valid-base64!");
        let token = creds.generate_token(1234567890);
        assert!(token.is_err());
    }

    #[test]
    fn test_deterministic_signing() {
        // Same nonce should produce same signature
        let creds = Credentials::new("test_key", "dGVzdF9zZWNyZXQ=");
        let token1 = creds.generate_token(1234567890).unwrap();
        let token2 = creds.generate_token(1234567890).unwrap();
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_different_nonces() {
        // Different nonces should produce different signatures
        let creds = Credentials::new("test_key", "dGVzdF9zZWNyZXQ=");
        let token1 = creds.generate_token(1234567890).unwrap();
        let token2 = creds.generate_token(9876543210).unwrap();
        assert_ne!(token1, token2);
    }
}
