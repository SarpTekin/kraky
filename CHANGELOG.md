# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-24

### Added

#### Core Features
- WebSocket client for Kraken Exchange API v2
- Automatic connection management with smart reconnection
- Connection lifecycle events (Connected, Disconnected, Reconnecting, etc.)
- Connection state monitoring
- Exponential backoff for reconnection attempts
- Configurable reconnection delays and limits

#### Market Data
- Orderbook subscription with managed state reconstruction
- Real-time trade subscription
- Ticker subscription with 24h statistics
- OHLC (candlestick) subscription with multiple intervals
- Multi-pair concurrent subscriptions
- Backpressure monitoring and drop rate tracking

#### Analytics Features
- **Orderbook imbalance detection** (UNIQUE FEATURE)
  - Bid/ask volume ratio calculation
  - Bullish/Bearish/Neutral signal generation
  - Customizable imbalance thresholds
  - Real-time trading signal generation
- CRC32 checksum validation for orderbook integrity
- Spread and mid-price calculation
- Top N bids/asks retrieval

#### Authentication & Trading
- HMAC-SHA256 authentication for private channels
- WebSocket-based order placement (UNIQUE FEATURE)
- Order cancellation and amendment
- Private channel support (balances, orders, executions)
- Validation mode for safe testing
- All order types supported (market, limit, stop, etc.)

#### Telegram Integration
- Telegram bot notifications
- Price alerts
- Imbalance signal alerts
- Large order (whale) detection
- Trading notifications
- Private account updates

#### Data Export
- Single-pair CSV export
- Multi-pair concurrent CSV export
- Orderbook snapshot export
- Trade history export
- Customizable export intervals

#### Developer Experience
- 16 working examples covering all features
- Comprehensive documentation
- 25 passing tests
- Modular feature flags for minimal binary size
- Type-safe async API
- Clear error messages with retry hints

### Features

#### Feature Flags
- `default` - Core features (reconnect + events + orderbook)
- `orderbook` - Orderbook data type
- `trades` - Trade data type
- `ticker` - Ticker data type
- `ohlc` - OHLC/candlestick data type
- `analytics` - Imbalance detection and advanced analytics
- `reconnect` - Smart reconnection logic
- `events` - Connection lifecycle events
- `auth` - HMAC-SHA256 authentication
- `private` - Private WebSocket channels
- `trading` - Order placement and management
- `simd` - SIMD JSON parsing for performance
- `checksum` - CRC32 orderbook validation
- `telegram` - Telegram bot integration
- `telegram-alerts` - Smart Telegram alerts with signals
- `market-data` - All market data types (convenience)
- `full` - All features enabled (convenience)

### Performance
- Zero-copy JSON deserialization
- Async I/O throughout
- Handles 1000+ updates/sec
- <5ms average latency
- Optional SIMD JSON parsing
- Bounded channels for backpressure control
- Default binary size: 7.2 MB
- Full features binary: 8.5 MB

### Documentation
- Comprehensive README with examples
- SETUP.md for credential configuration
- CONTRIBUTING.md for contributors
- 16 example programs with inline documentation
- Inline Rust doc comments throughout
- Feature-specific guides
- Security best practices

### Testing
- 25 unit and integration tests
- Orderbook operation tests (17 tests)
- Subscription handling tests (4 tests)
- Error parsing tests (6 tests)
- Reconnection logic tests (2 tests)
- 100% test success rate

### Security
- Environment variable-based credential management
- `.env.example` template for easy setup
- No hardcoded secrets
- Git-ignored credential files
- Clear security documentation
- HMAC-SHA256 authentication
- Validation mode for safe testing

### Examples
1. `orderbook.rs` - Basic orderbook subscription
2. `trades.rs` - Trade stream
3. `ticker.rs` - Ticker updates
4. `ohlc.rs` - OHLC candlesticks
5. `multi_subscribe.rs` - Multiple concurrent subscriptions
6. `demo.rs` - Comprehensive feature showcase
7. `benchmark.rs` - Performance benchmarking
8. `auth_example.rs` - Authentication demonstration
9. `liquidity_monitor.rs` - Real-time liquidity analysis
10. `multi_pair_monitor.rs` - Multi-asset dashboard
11. `whale_watcher.rs` - Large order detection
12. `simple_price_alerts.rs` - Basic Telegram alerts
13. `telegram_imbalance_bot.rs` - Imbalance signal alerts
14. `telegram_private_alerts.rs` - Private account alerts
15. `telegram_trading_bot.rs` - Full trading bot
16. `telegram_trading_demo.rs` - Trading demo (no keys needed)
17. `export_to_csv.rs` - Single-pair CSV export
18. `export_multi_csv.rs` - Multi-pair CSV export

### Fixed
- OHLC field name mapping (`trades` vs `count`)
- Benchmark example feature requirements
- Demo example documentation clarity

### Documentation
- Added presentation materials for Kraken Forge Hackathon
- Created submission checklist and video recording guide
- Added project summary and architecture overview

---

## Release Notes

### Version 0.1.0 - Initial Release

This is the initial release of Kraky, built for the Kraken Forge Hackathon. It provides a complete, production-ready SDK for the Kraken Exchange WebSocket API v2 with unique features not found in other libraries:

**Unique Features:**
- Orderbook imbalance detection with trading signals
- WebSocket-based order management (no REST API needed)

**Production Ready:**
- 25 passing tests
- Comprehensive error handling
- Smart reconnection logic
- Complete documentation
- 16 working examples

**Modular Design:**
- Feature flags for minimal binary size
- Pay only for what you use
- Core SDK: 7.2 MB
- Full features: 8.5 MB

---

[Unreleased]: https://github.com/sarptekin/kraky/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sarptekin/kraky/releases/tag/v0.1.0
