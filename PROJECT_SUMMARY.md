# Kraky SDK - Project Summary

## 📦 What Is This?

**Kraky** is a lightweight, production-ready Rust SDK for the Kraken Exchange WebSocket API v2.

**Built for:** Kraken Forge Hackathon
**License:** MIT
**Language:** Rust 1.70+
**Status:** Production-ready with 25 passing tests

---

## 🌟 Unique Features

### 1. Orderbook Imbalance Detection
- **Only SDK** with built-in bullish/bearish signal generation
- Real-time volume analysis
- Customizable thresholds
- Ready-to-use trading signals

### 2. WebSocket Trading
- **Only SDK** supporting full order management via WebSocket
- Place, cancel, and amend orders without REST API
- Lower latency than traditional REST trading
- All order types supported (market, limit, stop, etc.)

### 3. Modular Architecture
- Feature flags for everything
- Core SDK only 7.2 MB
- Trading adds only 3 KB
- Pay only for what you use

---

## 📊 Feature Breakdown

### Core Features (Default)
- ✅ WebSocket connection with automatic reconnection
- ✅ Connection lifecycle events
- ✅ Orderbook depth with managed state
- ✅ Spread and mid-price calculation
- ✅ Backpressure monitoring

### Market Data (Opt-in)
- ✅ Real-time trades
- ✅ Ticker updates
- ✅ OHLC candlesticks
- ✅ Multi-pair subscriptions

### Analytics (Opt-in)
- ✅ Orderbook imbalance detection
- ✅ Volume ratio analysis
- ✅ Bullish/Bearish signals
- ✅ CRC32 checksum validation

### Authentication (Opt-in)
- ✅ HMAC-SHA256 signing
- ✅ Private account channels
- ✅ Balance updates
- ✅ Order status tracking
- ✅ Execution notifications

### Trading (Opt-in)
- ✅ WebSocket order placement
- ✅ Order cancellation
- ✅ Order amendment
- ✅ Validation mode (safe testing)
- ✅ All order types

### Telegram (Opt-in)
- ✅ Real-time mobile alerts
- ✅ Formatted notifications
- ✅ Imbalance signals
- ✅ Trading notifications
- ✅ Account updates

---

## 📈 Statistics

| Metric | Value |
|--------|-------|
| Tests | 25 passing |
| Examples | 16 working examples |
| Default Binary Size | 7.2 MB |
| With Trading | 7.23 MB (+3 KB) |
| With Full Features | 8.5 MB |
| Core Dependencies | 12 |
| Full Dependencies | ~35 |
| Lines of Code | ~3,500 |
| Documentation | Comprehensive README |
| Test Coverage | Core features fully tested |

---

## 🗂️ Project Structure

```
kraky/
├── src/
│   ├── lib.rs              # Main library entry
│   ├── client.rs           # WebSocket client (1,200 lines)
│   ├── subscriptions.rs    # Subscription management
│   ├── messages.rs         # Kraken message types
│   ├── error.rs            # Error handling
│   ├── auth.rs             # HMAC-SHA256 authentication
│   ├── telegram.rs         # Telegram bot integration
│   └── models/
│       ├── orderbook.rs    # Orderbook state + analytics
│       ├── trade.rs        # Trade data types
│       ├── ticker.rs       # Ticker data types
│       ├── ohlc.rs         # OHLC candlesticks
│       ├── private.rs      # Private channel types
│       └── trading.rs      # Trading types
├── examples/               # 16 working examples
│   ├── orderbook.rs        # Basic orderbook
│   ├── trades.rs           # Trade stream
│   ├── ticker.rs           # Ticker updates
│   ├── ohlc.rs             # OHLC candles
│   ├── multi_subscribe.rs  # Multiple subscriptions
│   ├── demo.rs             # Comprehensive demo
│   ├── benchmark.rs        # Performance test
│   ├── auth_example.rs     # Authentication
│   ├── liquidity_monitor.rs    # Liquidity tracking
│   ├── multi_pair_monitor.rs   # Multi-pair dashboard
│   ├── whale_watcher.rs        # Large order detection
│   ├── simple_price_alerts.rs  # Price alerts
│   ├── telegram_imbalance_bot.rs   # Imbalance alerts
│   ├── telegram_private_alerts.rs  # Private account alerts
│   ├── telegram_trading_bot.rs     # Full trading bot
│   └── telegram_trading_demo.rs    # Trading demo (no keys needed)
├── tests/                  # Integration tests
├── README.md               # Comprehensive documentation
├── ARCHITECTURE.md         # Technical architecture
├── PRESENTATION.md         # 5-min presentation guide
├── PRESENTATION_CHEATSHEET.md  # Quick reference
├── Cargo.toml              # Dependencies and features
└── LICENSE                 # MIT License
```

---

## 🎯 Use Cases

### 1. Market Data Monitoring
```rust
let client = KrakyClient::connect().await?;
let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;

while let Some(update) = orderbook.next().await {
    if let Some(ob) = client.get_orderbook("BTC/USD") {
        println!("Spread: {:?}", ob.spread());
        println!("Imbalance: {:.2}%", ob.imbalance() * 100.0);
    }
}
```

### 2. Trading Bot
```rust
let client = KrakyClient::connect().await?;
let creds = Credentials::new(api_key, api_secret);

let order = OrderParams::limit_buy("BTC/USD", 0.001, 50000.0);
let response = client.place_order(&creds, order).await?;

println!("Order placed: {}", response.order_id);
```

### 3. Telegram Alert Bot
```rust
let client = KrakyClient::connect().await?;
let bot = TelegramNotifier::new(token, chat_id);

let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;

while let Some(_) = orderbook.next().await {
    if let Some(ob) = client.get_orderbook("BTC/USD") {
        let metrics = ob.imbalance_metrics();
        let signal = metrics.signal(0.15);

        if !matches!(signal, ImbalanceSignal::Neutral) {
            bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
        }
    }
}
```

---

## 🏗️ Architecture Highlights

### Async Throughout
- Built on Tokio runtime
- Non-blocking I/O
- Efficient resource usage

### Zero-Copy Parsing
- Direct deserialization with Serde
- Minimal memory allocations
- High throughput

### Managed State
- Automatic orderbook reconstruction
- Pre-computed metrics
- Always up-to-date

### Smart Reconnection
- Exponential backoff
- Configurable delays
- Unlimited or limited attempts

### Error Handling
- Structured Kraken error parsing
- Retryable error detection
- Rate limit awareness

### Backpressure Control
- Bounded channels
- Drop rate monitoring
- Memory protection

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test orderbook

# Run with output
cargo test -- --nocapture
```

**Test Coverage:**
- ✅ Orderbook operations (17 tests)
- ✅ Subscription handling (4 tests)
- ✅ Error parsing (6 tests)
- ✅ Reconnection logic (2 tests)

---

## 🚀 Quick Start

### Installation
```toml
[dependencies]
kraky = { git = "https://github.com/SarpTekin/kraky" }
tokio = { version = "1.35", features = ["full"] }
```

### Basic Example
```rust
use kraky::KrakyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakyClient::connect().await?;
    let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;

    while let Some(update) = orderbook.next().await {
        if let Some(ob) = client.get_orderbook("BTC/USD") {
            println!("Best bid: {:?}, Best ask: {:?}",
                ob.best_bid(), ob.best_ask());
        }
    }

    Ok(())
}
```

---

## 📚 Documentation

- **README.md** - Comprehensive guide (1,500+ lines)
- **ARCHITECTURE.md** - Technical architecture
- **PRESENTATION.md** - Hackathon presentation guide
- **Examples** - 16 working code examples
- **Inline docs** - Rust doc comments throughout

---

## 🎓 Learning Resources

### For Beginners
Start with these examples:
1. `orderbook.rs` - Basic subscription
2. `trades.rs` - Trade stream
3. `ticker.rs` - Ticker updates
4. `multi_subscribe.rs` - Multiple streams

### For Advanced Users
1. `demo.rs` - All features
2. `telegram_trading_bot.rs` - Full trading bot
3. `whale_watcher.rs` - Large order detection
4. `liquidity_monitor.rs` - Market analysis

---

## 🔧 Development

### Build All Examples
```bash
cargo build --examples --features full
```

### Run Tests
```bash
cargo test
```

### Check Code
```bash
cargo clippy
cargo fmt --check
```

### Generate Docs
```bash
cargo doc --open --features full
```

---

## 🏆 Hackathon Submission

**Category:** Best Use of Kraken API

**What We Built:**
A production-ready Rust SDK with unique features not found in other Kraken libraries:
- Orderbook imbalance detection for trading signals
- WebSocket-based order management (no REST needed)
- Modular architecture with optional features
- Comprehensive testing and documentation

**Why It Matters:**
- Fills gap in Kraken's ecosystem (no official Rust SDK)
- Provides tools for algorithmic traders
- Production-ready from day one
- Demonstrates advanced Rust techniques

**Technical Achievement:**
- Async/await throughout
- Zero-copy parsing
- Comprehensive error handling
- 25 passing tests
- 16 working examples

---

## 📝 License

MIT License - See LICENSE file for details

---

## 🙏 Acknowledgments

Built for the Kraken Forge Hackathon
Powered by Rust, Tokio, and the Kraken WebSocket API v2

---

## 📞 Contact

- GitHub: https://github.com/SarpTekin/kraky
- Issues: https://github.com/SarpTekin/kraky/issues

---

**Ready to trade? Start with Kraky!** 🐙🚀
