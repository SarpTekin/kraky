# Kraky 🐙

A lightweight, production-ready Rust SDK for the [Kraken Exchange](https://www.kraken.com/) WebSocket API v2 with built-in orderbook imbalance detection and Telegram alert integration.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Documentation](https://img.shields.io/badge/docs-docs.rs-blue)](https://docs.rs/kraky)

---

## Why Kraky?

Building with cryptocurrency exchange WebSocket APIs is complex. Kraky handles the hard parts:

- **🔄 Connection Management** - Smart reconnection with exponential backoff
- **📊 State Synchronization** - Automatic orderbook reconstruction from incremental updates
- **📈 Trading Signals** - Built-in orderbook imbalance detection (bullish/bearish)
- **🤖 Telegram Alerts** - Real-time notifications on your phone
- **🔐 WebSocket Trading** - Place orders without REST API
- **⚡ Lightweight & Modular** - Feature flags let you compile only what you need (7.2-8.5 MB)

---

## 🏆 For Hackathon Judges

**Test the SDK in under 2 minutes - no credentials needed!**

### Prerequisites
- Rust 1.70+ ([install here](https://rustup.rs/))

### Quick Start

```bash
# Clone and test
git clone https://github.com/SarpTekin/kraky.git
cd kraky

# Run tests (69 passing)
cargo test

# Run the demo - shows all core features
cargo run --example demo --features full
```

### What You'll See

The demo showcases:
- ✅ WebSocket connection to Kraken
- ✅ Real-time orderbook updates
- ✅ Trade stream
- ✅ Ticker data
- ✅ Orderbook imbalance detection (trading signals)
- ✅ Connection event callbacks
- ✅ Backpressure monitoring

### More Examples

```bash
# Basic examples (no credentials needed)
cargo run --example orderbook                    # Orderbook depth
cargo run --example trades --features trades     # Real-time trades
cargo run --example ticker --features ticker     # Price/volume updates

# Advanced examples (require Telegram/Kraken credentials - see SETUP.md)
cargo run --example whale_watcher --features telegram-alerts              # Whale detection
cargo run --example telegram_imbalance_bot --features telegram-alerts     # Imbalance alerts
cargo run --example telegram_trading_bot --features telegram,trading      # Trading bot
```

**📝 Need credentials?** See [SETUP.md](SETUP.md) for Telegram and Kraken API setup.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kraky = { git = "https://github.com/SarpTekin/kraky" }
tokio = { version = "1.35", features = ["full"] }
```

### Feature Flags

Kraky uses feature flags for modular compilation:

```toml
# Default (orderbook + reconnect + events)
kraky = { git = "..." }

# With analytics (imbalance detection)
kraky = { git = "...", features = ["analytics"] }

# With Telegram alerts
kraky = { git = "...", features = ["telegram-alerts"] }

# Everything (market data + analytics + telegram + trading)
kraky = { git = "...", features = ["full"] }
```

**Available features:**
- `orderbook`, `trades`, `ticker`, `ohlc` - Market data types
- `analytics` - Orderbook imbalance detection
- `telegram`, `telegram-alerts` - Telegram bot integration
- `auth`, `private`, `trading` - Authentication and trading
- `checksum` - CRC32 orderbook validation
- `simd` - SIMD-accelerated JSON parsing

See [docs.rs](https://docs.rs/kraky) for complete feature documentation.

---

## Quick Start

```rust
use kraky::KrakyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Kraken WebSocket
    let client = KrakyClient::connect().await?;

    // Subscribe to BTC/USD orderbook (depth: 10 levels)
    let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;

    // Process real-time updates
    while let Some(update) = orderbook.next().await {
        // Access managed orderbook state
        if let Some(ob) = client.get_orderbook("BTC/USD") {
            println!("Best bid: ${:.2}", ob.best_bid().unwrap_or(0.0));
            println!("Best ask: ${:.2}", ob.best_ask().unwrap_or(0.0));
            println!("Spread: ${:.2}", ob.spread().unwrap_or(0.0));

            // Get trading signal (requires 'analytics' feature)
            #[cfg(feature = "analytics")]
            {
                let imbalance = ob.imbalance();
                let signal = ob.imbalance_signal();
                println!("Imbalance: {:.2}% - Signal: {:?}",
                    imbalance * 100.0, signal);
            }
        }
    }

    Ok(())
}
```

---

## Examples

Kraky includes **18 working examples**:

**Basic (No Credentials):**
- `orderbook.rs` - Orderbook depth updates
- `trades.rs` - Real-time trade stream
- `ticker.rs` - Price and volume data
- `ohlc.rs` - Candlestick data
- `multi_subscribe.rs` - Multiple concurrent subscriptions
- `demo.rs` - Comprehensive feature showcase

**Advanced (Requires Setup):**
- `telegram_imbalance_bot.rs` - Orderbook imbalance alerts
- `whale_watcher.rs` - Large order detection
- `simple_price_alerts.rs` - Price threshold alerts
- `telegram_private_alerts.rs` - Account activity notifications
- `telegram_trading_bot.rs` - Automated trading with alerts
- `export_to_csv.rs` - Market data export
- And more...

See [examples/](examples/) directory for all examples.

---

## Documentation

- **📚 API Documentation**: [docs.rs/kraky](https://docs.rs/kraky) - Complete API reference
- **🔧 Setup Guide**: [SETUP.md](SETUP.md) - Telegram and Kraken API credentials
- **💡 Examples**: [examples/](examples/) - 18 working examples with explanations

---

## Features

### Core Features (Always Included)
- ✅ WebSocket connection management
- ✅ Automatic reconnection with exponential backoff
- ✅ Connection lifecycle events
- ✅ Managed orderbook state
- ✅ Type-safe API with zero-copy parsing

### Market Data (Opt-in)
- 📊 Orderbook depth (default)
- 💱 Real-time trades
- 📈 Ticker updates
- 📉 OHLC/candlestick data

### Analytics (Opt-in)
- 🎯 Orderbook imbalance detection
- 📊 Bullish/Bearish signal generation
- ✅ CRC32 checksum validation
- ⚡ SIMD-accelerated JSON parsing

### Integration (Opt-in)
- 🤖 Telegram bot notifications
- 🐋 Whale detection alerts
- 💰 Price threshold alerts
- 🔐 Private account data (balances, orders, executions)
- 💸 WebSocket trading (place/cancel/amend orders)

---

## Architecture

Kraky is built with:
- **Tokio** - Async runtime for efficient I/O
- **Tokio-Tungstenite** - WebSocket client
- **Serde** - Zero-copy JSON parsing
- **Feature Flags** - Modular compilation
- **BTreeMap** - Ordered orderbook storage (O(log n) operations)

**Binary Size:**
- Minimal (orderbook only): ~7.2 MB
- Full featured: ~8.5 MB
- Each data type adds only 40-50 KB

See [docs.rs/kraky](https://docs.rs/kraky) for detailed architecture documentation.

---

## Testing

```bash
# Run all tests
cargo test --all-features

# Results: 69 tests passing
# - 47 unit tests
# - 22 doctests
```

---

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Ensure `cargo test --all-features` passes
5. Submit a pull request

---

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Links

- **Documentation**: [docs.rs/kraky](https://docs.rs/kraky)
- **Repository**: [github.com/SarpTekin/kraky](https://github.com/SarpTekin/kraky)
- **Setup Guide**: [SETUP.md](SETUP.md)
- **Examples**: [examples/](examples/)

---

**Built for the Kraken Forge Hackathon** 🐙
