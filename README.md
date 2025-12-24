# Kraky 🐙

A lightweight, high-performance Rust SDK for connecting to the [Kraken Exchange](https://www.kraken.com/) WebSocket API v2.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

---

## 🏆 For Hackathon Judges

**Quick test in under 2 minutes:**

### Prerequisites
- Rust 1.70+ installed ([install here](https://rustup.rs/))

### Run the Demo

```bash
# Clone the repo
git clone https://github.com/SarpTekin/kraky.git
cd kraky

# Run all tests (29 tests)
cargo test

# ⭐ RECOMMENDED: Run the comprehensive demo (shows all features)
cargo run --example demo --features full

# Basic examples (single data type):
cargo run --example orderbook                    # Live orderbook depth (default feature)
cargo run --example trades --features trades     # Real-time trades
cargo run --example ticker --features ticker     # Ticker updates
cargo run --example ohlc --features ohlc         # OHLC candles

# Multi-subscription examples:
cargo run --example multi_subscribe --features trades,ticker  # Multiple subscriptions
cargo run --example benchmark --features orderbook,trades     # Performance benchmark

# Advanced examples:
cargo run --example whale_watcher --features telegram-alerts              # 🐋 Whale detection
cargo run --example liquidity_monitor --features analytics                # 💧 Liquidity tracking
cargo run --example multi_pair_monitor --features market-data,analytics   # 📊 Multi-pair dashboard
cargo run --example simple_price_alerts --features telegram-alerts        # 🔔 Price alerts
cargo run --example telegram_imbalance_bot --features telegram-alerts     # 🤖 Imbalance bot
cargo run --example export_to_csv --features trades,analytics             # 📊 Export to CSV files

# Authentication examples (requires API credentials):
cargo run --example auth_example --features private                       # 🔐 Authentication
cargo run --example telegram_private_alerts --features telegram,private   # 📱 Private alerts

# Trading examples:
cargo run --example telegram_trading_demo --features telegram,trading     # 🎯 Trading demo (NO credentials needed!)
cargo run --example telegram_trading_bot --features telegram,trading      # 💰 Full trading bot (requires API credentials)
```

### What You'll See

**Demo Example Output (abbreviated):**
```
╔══════════════════════════════════════════════════════════════╗
║           🐙 KRAKY SDK DEMO - Kraken Forge Hackathon         ║
╚══════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════
  FEATURE 1: WebSocket Connection
═══════════════════════════════════════════════════════════════
📡 Connecting to Kraken WebSocket API...
✅ Connected!

═══════════════════════════════════════════════════════════════
  FEATURE 2: Connection Events
═══════════════════════════════════════════════════════════════
📌 Subscribed to connection events
   Events: Connected, Disconnected, Reconnecting, Reconnected...

═══════════════════════════════════════════════════════════════
  FEATURE 3: Connection State
═══════════════════════════════════════════════════════════════
   Current state: ✅ Connected
   is_connected(): true
   Reconnect Config: enabled, 500ms initial, 30s max, 2.0x backoff

═══════════════════════════════════════════════════════════════
  FEATURE 5: Live Market Data (15 seconds)
═══════════════════════════════════════════════════════════════
📖 ORDERBOOK UPDATE #1
   Best Bid: $97234.50 | Best Ask: $97235.00
   Spread: $0.50 | Mid: $97234.75
🟢 TRADE: Buy 0.050000 BTC @ $97235.00
📈 TICKER: $97235.00 (24h: +2.35%) Vol: 1234.56 BTC

═══════════════════════════════════════════════════════════════
  FEATURE 6: Backpressure Monitoring
═══════════════════════════════════════════════════════════════
Backpressure stats (delivered / dropped / drop rate):
   📖 Orderbook: 47 / 0 / 0.00%
   💱 Trades:    23 / 0 / 0.00%

═══════════════════════════════════════════════════════════════
  FEATURE 7: Orderbook Checksum Validation (CRC32)
═══════════════════════════════════════════════════════════════
   Calculated Checksum: 0x1A2B3C4D
   Checksum Valid:      ✅ Yes

═══════════════════════════════════════════════════════════════
  FEATURE 8: Orderbook Imbalance Detection
═══════════════════════════════════════════════════════════════
   ┌─────────────────────────────────────┐
   │  Bid Volume:   12.3456 BTC          │
   │  Ask Volume:   8.7654 BTC           │
   │  Imbalance:     +17.02%             │
   │  Signal:       🟢 BULLISH           │
   └─────────────────────────────────────┘

╔══════════════════════════════════════════════════════════════╗
║                    🎉 DEMO COMPLETE!                          ║
╠══════════════════════════════════════════════════════════════╣
║  Features Demonstrated:                                       ║
║    ✅ WebSocket Connection                                    ║
║    ✅ Connection Events (lifecycle callbacks)                 ║
║    ✅ Connection State Monitoring                             ║
║    ✅ Multiple Subscriptions                                  ║
║    ✅ Backpressure Monitoring                                 ║
║    ✅ Orderbook Checksum Validation                           ║
║    ✅ Orderbook Imbalance Detection                           ║
║    ✅ Managed Orderbook State                                 ║
╚══════════════════════════════════════════════════════════════╝
```

### Key Features Demonstrated in Demo

| Feature # | What It Shows |
|-----------|---------------|
| 1 | WebSocket Connection |
| 2 | Connection Events (lifecycle callbacks) |
| 3 | Connection State Monitoring |
| 4 | Multiple Subscriptions (orderbook, trades, ticker) |
| 5 | Real-time Market Data Processing |
| 6 | Backpressure Monitoring |
| 7 | Orderbook Checksum Validation (CRC32) |
| 8 | Orderbook Imbalance Detection |
| 9 | Managed Orderbook State |

### Examples Quick Reference

| Example | Required Features | What It Shows |
|---------|------------------|---------------|
| `orderbook` | _(default)_ | Real-time depth, managed state, spread calculation |
| `trades` | `trades` | Live trade stream, buy/sell sides |
| `ticker` | `ticker` | Price, volume, 24h change |
| `ohlc` | `ohlc` | Candlestick data for charting |
| `multi_subscribe` | `trades,ticker` | Concurrent subscriptions with `tokio::select!` |
| `benchmark` | `orderbook,trades` | Performance testing |
| `telegram_imbalance_bot` | `telegram-alerts` | 🤖 Real-time Telegram alerts with imbalance signals |
| `auth_example` | `private` | 🔐 HMAC-SHA256 authentication for private channels |
| `telegram_private_alerts` | `telegram,private` | 📱 Private account alerts (balances, orders, executions) |
| `whale_watcher` | `telegram-alerts` | 🐋 Large order detection with Telegram notifications |
| `multi_pair_monitor` | `market-data,analytics` | 📊 Monitor multiple trading pairs simultaneously |
| `liquidity_monitor` | `analytics` | 💧 Track market liquidity and spread changes |
| `simple_price_alerts` | `telegram-alerts` | 🔔 Beginner-friendly price threshold alerts |
| `export_to_csv` | `trades,analytics` | 📊 Export live market data to CSV for analysis |
| `telegram_trading_demo` | `telegram,trading` | 🎯 Trading demo - NO credentials needed! Shows all notifications |
| `telegram_trading_bot` | `telegram,trading` | 💰 Complete trading bot with order management (needs API keys) |
| `demo` | `full` | ⭐ Comprehensive showcase of all features |

---

## Features

- **Real-time Market Data**: Stream orderbook, trades, tickers, and OHLC candles
- **Managed Orderbook State**: Automatic reconstruction from incremental updates
- **Orderbook Imbalance Detection**: Built-in bullish/bearish signal generation
- **Orderbook Checksum Validation**: CRC32 validation to detect data corruption
- **🔐 Authenticated WebSocket**: HMAC-SHA256 authentication for private channels (optional)
- **Private Account Data**: Real-time balance, order, and execution updates (optional)
- **💰 Trading via WebSocket**: Place, cancel, and manage orders - no REST API needed (optional)
- **🤖 Telegram Bot Integration**: Real-time alerts with imbalance signals (optional)
- **Smart Reconnection**: Automatic reconnection with exponential backoff
- **Connection Events**: Subscribe to connect/disconnect/reconnect lifecycle events
- **Type-safe API**: Strongly typed models for all Kraken message types
- **Async/Await**: Built on Tokio for efficient async I/O
- **Zero-copy Parsing**: Efficient JSON deserialization with Serde
- **Backpressure Control**: Bounded channels prevent memory issues
- **Kraken Error Parsing**: Structured parsing of Kraken API errors
- **Automatic Heartbeat**: Built-in ping/pong handling

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kraky = { git = "https://github.com/SarpTekin/kraky" }
tokio = { version = "1.35", features = ["full"] }
```

## 🎛️ Feature Flags - Choose What You Need

Kraky uses **feature flags** to keep your binary lightweight. Only compile what you actually use!

### 🧭 Quick Decision Guide

```
┌─────────────────────────────────────────────────────────────────┐
│  What do you want to do?                                         │
└─────────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
   📊 Market Data        🔐 Private Data       💰 Trading
        │                     │                     │
        ├─ orderbook ✓        ├─ private            ├─ trading
        ├─ trades             │   (adds auth)        │   (adds auth + private)
        ├─ ticker             │                     │
        └─ ohlc               └─ 📱 + telegram      └─ 📱 + telegram
                                   (notifications)        (trade alerts)

    📈 Analytics?          🤖 Telegram Alerts?      ⚡ Performance?
        │                     │                     │
        ├─ analytics          ├─ telegram-alerts    ├─ simd
        └─ checksum           │   (auto-includes    └─ (2-3x faster JSON)
                              │    telegram,
                              │    analytics,
                              │    ticker)
```

### 📦 Default Features (Always Included)

When you add Kraky with no feature flags, you get:

```toml
kraky = { git = "https://github.com/SarpTekin/kraky" }
```

**Includes:**
- ✅ `reconnect` - Smart reconnection with exponential backoff
- ✅ `events` - Connection lifecycle event callbacks
- ✅ `orderbook` - Orderbook depth subscription and managed state

**Binary size:** ~7.2 MB
**Dependencies added:** 0 (core only)

---

### 📊 Data Type Features (Opt-in)

Choose which market data types you need:

| Feature | What You Get | Added Size | Dependencies |
|---------|-------------|------------|--------------|
| `trades` | Real-time trade execution stream | +50 KB | 0 |
| `ticker` | Price, volume, 24h stats | +45 KB | 0 |
| `ohlc` | Candlestick/OHLC data | +40 KB | 0 |
| `orderbook` | Depth updates _(included by default)_ | ✓ | 0 |

**Meta feature:** `market-data` = all of the above

```toml
# Just trades
kraky = { git = "...", features = ["trades"] }

# All market data types
kraky = { git = "...", features = ["market-data"] }
```

---

### 🎯 Analytics Features (Opt-in)

Advanced orderbook analysis:

| Feature | What You Get | Added Size | Requires |
|---------|-------------|------------|----------|
| `analytics` | Imbalance detection, signals | +25 KB | `orderbook` |
| `checksum` | CRC32 orderbook validation | +15 KB | `orderbook` |

**Dependencies added:**
- `analytics`: 0 additional deps
- `checksum`: +1 dep (`crc32fast`)

```toml
# Orderbook with analytics
kraky = { git = "...", features = ["analytics"] }

# With checksum validation too
kraky = { git = "...", features = ["analytics", "checksum"] }
```

---

### 🔐 Authentication & Private Data

Access your account data and trade via WebSocket:

```
┌──────────────────────────────────────────────────────┐
│  Feature Dependency Chain                             │
├──────────────────────────────────────────────────────┤
│                                                       │
│  auth  ──>  private  ──>  trading                    │
│   │            │             │                        │
│   │            │             └─> Place/cancel orders  │
│   │            └──> Balances, orders, executions      │
│   └────> HMAC-SHA256 signing                         │
│                                                       │
└──────────────────────────────────────────────────────┘
```

| Feature | What You Get | Added Size | Dependencies |
|---------|-------------|------------|--------------|
| `auth` | HMAC-SHA256 authentication | +50 KB | +3 (`hmac`, `sha2`, `base64`) |
| `private` | Balance/order/execution updates | +0 KB | Includes `auth` |
| `trading` | Place/cancel/amend orders | +3 KB | Includes `auth` + `private` |

**Example usage:**

```toml
# Private account data
kraky = { git = "...", features = ["private"] }
# Automatically includes: auth

# Full trading capabilities
kraky = { git = "...", features = ["trading"] }
# Automatically includes: auth + private
```

---

### 📱 Telegram Integration

Real-time notifications via Telegram bot:

| Feature | What You Get | Added Size | Requires |
|---------|-------------|------------|----------|
| `telegram` | Basic Telegram notifications | +800 KB | None |
| `telegram-alerts` | Smart alerts with imbalance signals | +800 KB | `telegram` + `analytics` + `ticker` |

**Dependencies added:** +1 (`teloxide`)

```toml
# Basic Telegram notifications
kraky = { git = "...", features = ["telegram"] }

# Smart alerts with orderbook signals
kraky = { git = "...", features = ["telegram-alerts"] }
# Auto-includes: telegram, analytics, ticker

# Telegram + Private account alerts
kraky = { git = "...", features = ["telegram", "private"] }

# Telegram + Trading alerts
kraky = { git = "...", features = ["telegram", "trading"] }
```

---

### ⚡ Performance Features

Boost JSON parsing speed:

| Feature | What You Get | Added Size | Trade-offs |
|---------|-------------|------------|------------|
| `simd` | SIMD-accelerated JSON (2-3x faster) | +100 KB | +15 dependencies |

**When to use:** High-frequency trading, processing thousands of updates/sec

```toml
kraky = { git = "...", features = ["full", "simd"] }
```

---

### 🎁 Meta Features (Convenience)

Bundles of commonly used features:

| Meta Feature | Includes | Use Case |
|-------------|----------|----------|
| `market-data` | `orderbook` + `trades` + `ticker` + `ohlc` | Comprehensive market monitoring |
| `full` | All features except `simd` | Everything you need |

```toml
# Everything (trading, analytics, telegram, private)
kraky = { git = "...", features = ["full"] }
```

---

### 🎯 Common Combinations

**Choose your use case:**

| I want to... | Features to enable | Why? |
|-------------|-------------------|------|
| 🎯 **Track orderbook** | `["orderbook"]` _(default)_ | Core features, no extras |
| 📈 **Detect imbalances** | `["analytics"]` | Adds signal generation |
| 🤖 **Build alert bot** | `["telegram-alerts"]` | Telegram + analytics + ticker |
| 🔐 **Monitor my account** | `["telegram", "private"]` | Balance/order alerts |
| 💰 **Build trading bot** | `["trading", "analytics"]` | Trade + signals |
| 📱 **Trading with alerts** | `["telegram", "trading"]` | Full trading notifications |
| 🐋 **Watch for whales** | `["telegram-alerts"]` | Large order detection |
| ⚡ **High-frequency trading** | `["trading", "simd"]` | Fast execution |
| 📊 **Multi-asset dashboard** | `["market-data", "analytics"]` | All data + analytics |
| 🎓 **Learning/testing** | `["orderbook"]` _(default)_ | Start simple |

---

### 📏 Binary Size Impact

**Stay lightweight - only pay for what you use:**

```
┌────────────────────────────────────────────────────┐
│  Configuration          Size      vs Full    Added │
├────────────────────────────────────────────────────┤
│  full (everything)      8.5 MB    baseline    —    │
│  market-data only       7.8 MB     -8%        —    │
│  orderbook + trading    7.25 MB    -15%      +53KB │
│  orderbook + private    7.23 MB    -15%      +50KB │
│  orderbook (default)    7.2 MB     -15%       —    │
│  trades only            6.9 MB     -19%       —    │
└────────────────────────────────────────────────────┘
```

**Key takeaway:**
- 🪶 Authentication adds only ~50 KB (0.6%)
- 🪶 Trading adds only ~3 KB on top of auth (0.04%)
- 🪶 The SDK remains lightweight even with full capabilities!

---

### 🔍 Dependency Count by Feature

Understanding what each feature pulls in:

```
Core (no features)              12 deps
  ├─ tokio, serde, serde_json, futures-util
  ├─ native-tls, tokio-tungstenite
  └─ thiserror, tracing, url, chrono, uuid, parking_lot

+ auth                          +3 deps  (hmac, sha2, base64)
+ checksum                      +1 dep   (crc32fast)
+ simd                          +15 deps (simd-json + dependencies)
+ telegram                      +1 dep   (teloxide - brings ~20 transitive)

Data types (trades/ticker/ohlc)  +0 deps (just code)
Analytics                        +0 deps (just code)
```

**Total with `full` feature:** ~30-35 dependencies
**Total with `telegram` + `full`:** ~50-55 dependencies
**Total minimal (trades only):** 12 dependencies

---

### 💡 Disabling Default Features

Want maximum control? Disable defaults and choose exactly what you need:

```toml
# Minimal - only trades, no reconnection or events
kraky = {
    git = "https://github.com/SarpTekin/kraky",
    default-features = false,
    features = ["trades"]
}

# Custom combination - orderbook + reconnect only
kraky = {
    git = "https://github.com/SarpTekin/kraky",
    default-features = false,
    features = ["orderbook", "reconnect"]
}
```

**Note:** Most users should keep the default features enabled for reliability.

---

## Quick Start

```rust
use kraky::KrakyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Kraken WebSocket
    let client = KrakyClient::connect().await?;
    
    // Subscribe to BTC/USD orderbook
    let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;
    
    // Process updates
    while let Some(update) = orderbook.next().await {
        println!("Orderbook update: {:?}", update.data[0].symbol);
        
        // Access managed orderbook state
        if let Some(ob) = client.get_orderbook("BTC/USD") {
            println!("Best bid: {:?}, Best ask: {:?}", ob.best_bid(), ob.best_ask());
            println!("Spread: {:?}", ob.spread());
        }
    }
    
    Ok(())
}
```

## Subscription Types

### Orderbook

Subscribe to real-time orderbook depth updates. The SDK maintains a local orderbook state that is automatically updated.

```rust
// Subscribe with depth of 10, 25, 100, 500, or 1000 levels
let mut subscription = client.subscribe_orderbook("BTC/USD", 10).await?;

while let Some(update) = subscription.next().await {
    // Raw update from Kraken
    for data in &update.data {
        println!("Symbol: {}", data.symbol);
        println!("Bids: {:?}", data.bids);
        println!("Asks: {:?}", data.asks);
    }
    
    // Or use managed orderbook state
    if let Some(orderbook) = client.get_orderbook("BTC/USD") {
        let top_bids = orderbook.top_bids(5);
        let top_asks = orderbook.top_asks(5);
        let spread = orderbook.spread();
        let mid_price = orderbook.mid_price();
    }
}
```

### Trades

Subscribe to real-time trade executions.

```rust
let mut trades = client.subscribe_trades("ETH/USD").await?;

while let Some(trade) = trades.next().await {
    println!("{} {} @ ${:.2}", 
        trade.side,      // Buy or Sell
        trade.qty,       // Trade quantity
        trade.price      // Trade price
    );
}
```

### Ticker

Subscribe to ticker updates with price and volume information.

```rust
let mut ticker = client.subscribe_ticker("BTC/USD").await?;

while let Some(tick) = ticker.next().await {
    println!("Last: ${:.2}", tick.last);
    println!("Bid: ${:.2} / Ask: ${:.2}", tick.bid, tick.ask);
    println!("24h Volume: {:.2}", tick.volume);
    println!("24h Change: {:.2}%", tick.change_pct);
}
```

### OHLC (Candlesticks)

Subscribe to OHLC candle updates for technical analysis.

```rust
use kraky::Interval;

// Available intervals: Min1, Min5, Min15, Min30, Hour1, Hour4, Day1, Week1, Day15
let mut ohlc = client.subscribe_ohlc("BTC/USD", Interval::Min1).await?;

while let Some(candle) = ohlc.next().await {
    println!("Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}",
        candle.open, candle.high, candle.low, candle.close
    );
    println!("Volume: {:.4}", candle.volume);
}
```

## Multiple Subscriptions

Subscribe to multiple streams and process them concurrently:

```rust
let mut btc_trades = client.subscribe_trades("BTC/USD").await?;
let mut eth_trades = client.subscribe_trades("ETH/USD").await?;
let mut btc_ticker = client.subscribe_ticker("BTC/USD").await?;

loop {
    tokio::select! {
        Some(trade) = btc_trades.next() => {
            println!("[BTC] Trade: {} @ {}", trade.qty, trade.price);
        }
        Some(trade) = eth_trades.next() => {
            println!("[ETH] Trade: {} @ {}", trade.qty, trade.price);
        }
        Some(ticker) = btc_ticker.next() => {
            println!("[BTC] Ticker: ${}", ticker.last);
        }
    }
}
```

## Authentication & Private Channels

Access private WebSocket channels for account data using HMAC-SHA256 authentication:

### Setup Credentials

```rust
use kraky::{KrakyClient, Credentials};

// Load credentials from environment variables
let api_key = std::env::var("KRAKEN_API_KEY")?;
let api_secret = std::env::var("KRAKEN_API_SECRET")?;

let credentials = Credentials::new(api_key, api_secret);

// Generate authentication token
let nonce = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)?
    .as_millis() as u64;

let token = credentials.generate_token(nonce)?;
```

### Private Data Types

#### Balance Updates

Monitor account balance changes in real-time:

```rust
pub struct BalanceUpdate {
    pub channel: String,
    pub update_type: String,
    pub data: Vec<BalanceData>,
}

pub struct BalanceData {
    pub asset: String,
    pub balance: f64,
    pub available: f64,
    pub hold: f64,
}
```

#### Order Updates

Track order lifecycle (placed, filled, cancelled):

```rust
pub struct OrderUpdate {
    pub channel: String,
    pub update_type: String,
    pub data: Vec<OrderData>,
}

pub struct OrderData {
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: f64,
    pub quantity: f64,
    pub filled: f64,
    pub status: String,
    pub timestamp: String,
}
```

#### Execution Updates

Receive trade execution notifications:

```rust
pub struct ExecutionUpdate {
    pub channel: String,
    pub update_type: String,
    pub data: Vec<ExecutionData>,
}

pub struct ExecutionData {
    pub execution_id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub quantity: f64,
    pub fee: f64,
    pub timestamp: String,
}
```

### Example: Private Channels with Telegram

See `examples/telegram_private_alerts.rs` for a complete bot that sends Telegram notifications for:
- Balance changes
- Order status updates
- Trade executions
- Portfolio summaries

```bash
export KRAKEN_API_KEY="your_api_key"
export KRAKEN_API_SECRET="your_api_secret"
export TELEGRAM_BOT_TOKEN="your_bot_token"
export TELEGRAM_CHAT_ID="your_chat_id"

cargo run --example telegram_private_alerts --features telegram,private
```

## Trading via WebSocket

**NEW:** Place, cancel, and manage orders directly via WebSocket - no REST API needed!

The SDK provides full trading capabilities while remaining lightweight (~3KB added).

### 🎯 Try the Demo (No Account Needed!)

Before diving into the code, see all trading features in action:

```bash
export TELEGRAM_BOT_TOKEN="your_bot_token"
export TELEGRAM_CHAT_ID="your_chat_id"

# Run the complete trading demo - NO Kraken credentials needed!
cargo run --example telegram_trading_demo --features telegram,trading
```

**This demo showcases:**
- ✅ All 7 types of trading notifications
- ✅ Order lifecycle (place → amend → cancel → fill)
- ✅ Real-time Telegram alerts
- ✅ Complete trading workflow simulation
- ✅ **Perfect for hackathon presentations!**

### Quick Start

```rust
use kraky::{KrakyClient, Credentials, OrderParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakyClient::connect().await?;
    let creds = Credentials::new("api_key", "api_secret");

    // Place a market buy order
    let order = OrderParams::market_buy("BTC/USD", 0.001);
    let response = client.place_order(&creds, order).await?;

    println!("Order placed: {}", response.order_id);
    Ok(())
}
```

### Order Types Supported

All via WebSocket API v2:

| Order Type | Description |
|------------|-------------|
| `Market` | Execute immediately at best price |
| `Limit` | Execute at specified price or better |
| `StopLoss` | Market order triggered at stop price |
| `StopLossLimit` | Limit order triggered at stop price |
| `TakeProfit` | Market order at profit target |
| `TakeProfitLimit` | Limit order at profit target |
| `TrailingStop` | Stop that follows price movement |
| `TrailingStopLimit` | Trailing stop with limit price |
| `Iceberg` | Hidden volume order |

### Trading Methods

#### Place Orders

```rust
// Market orders
let buy = OrderParams::market_buy("BTC/USD", 0.1);
let sell = OrderParams::market_sell("BTC/USD", 0.1);

// Limit orders
let buy = OrderParams::limit_buy("BTC/USD", 0.1, 50000.0);
let sell = OrderParams::limit_sell("BTC/USD", 0.1", 55000.0);

// With advanced options
let order = OrderParams::limit_buy("BTC/USD", 0.1, 50000.0)
    .with_time_in_force(TimeInForce::IOC)
    .with_post_only(true)
    .with_client_id("my-order-123")
    .with_stp(SelfTradePrevention::CancelNewest);

// Place the order
let response = client.place_order(&creds, order).await?;
println!("Order ID: {}", response.order_id);
```

#### Cancel Orders

```rust
// Cancel single order
client.cancel_order(&creds, "order-id-123").await?;

// Cancel all orders
let response = client.cancel_all_orders(&creds).await?;
println!("Cancelled {} orders", response.count);
```

#### Amend Orders

```rust
use kraky::AmendOrderParams;

let amend = AmendOrderParams {
    order_id: "order-123".to_string(),
    order_qty: Some(0.2),        // Change quantity
    limit_price: Some(51000.0),  // Change price
    trigger_price: None,
};

client.amend_order(&creds, amend).await?;
```

### Validation Mode (Safe Testing)

Test orders without executing them:

```rust
let order = OrderParams::market_buy("BTC/USD", 0.1)
    .with_validate(true);  // Order will be validated but NOT executed

let response = client.place_order(&creds, order).await?;
// Returns validation result without placing real order
```

Perfect for:
- Testing your bot logic
- Hackathon demonstrations
- Development without risk

### Telegram Trading Notifications

Get instant alerts for all trading events:

```rust
use kraky::TelegramNotifier;

let bot = TelegramNotifier::new("bot_token", chat_id);

// Order placed
bot.send_order_placed(&response, &params).await?;

// Order filled
bot.send_order_filled("BTC/USD", &OrderSide::Buy, 0.1, 50000.0, "order-123").await?;

// Order cancelled
bot.send_order_cancelled("BTC/USD", "order-123", Some("User request")).await?;

// Order failed
bot.send_order_failed(&params, "Insufficient funds").await?;

// Daily summary
bot.send_trading_summary(5, 1250.0, 45.75, 80.0).await?;
```

### Complete Trading Examples

We provide **2 examples** to help you get started:

#### 1. Trading Demo (No Credentials Needed!) 🎯

Perfect for learning and hackathon presentations:

```bash
export TELEGRAM_BOT_TOKEN="your_bot_token"
export TELEGRAM_CHAT_ID="your_chat_id"

cargo run --example telegram_trading_demo --features telegram,trading
```

**Features:**
- ✅ Demonstrates all 7 notification types
- ✅ Shows complete trading workflow
- ✅ **No Kraken API credentials required**
- ✅ Perfect for presentations and demos

#### 2. Real Trading Bot (Requires API Keys) 💰

See `examples/telegram_trading_bot.rs` for a full implementation featuring:
- Market and limit order placement
- Order cancellation and amendment
- Real-time Telegram notifications
- Error handling
- Validation mode (safe testing)

```bash
# Set your API credentials
export KRAKEN_API_KEY="your_api_key"
export KRAKEN_API_SECRET="your_api_secret"
export TELEGRAM_BOT_TOKEN="your_bot_token"
export TELEGRAM_CHAT_ID="your_chat_id"

# Run in validation mode (safe - no real trades)
cargo run --example telegram_trading_bot --features telegram,trading

# Enable real trading (use with caution!)
ENABLE_REAL_TRADING=true cargo run --example telegram_trading_bot --features telegram,trading
```

### API Reference (Trading)

| Method | Description |
|--------|-------------|
| `place_order(creds, params)` | Place a new order |
| `cancel_order(creds, order_id)` | Cancel an order |
| `cancel_all_orders(creds)` | Cancel all open orders |
| `amend_order(creds, params)` | Modify an existing order |

All trading operations use WebSocket API v2 - no REST calls needed!

## Error Handling

### Kraken Error Parsing

The SDK parses Kraken's error format (`SeverityCategory:Message`) into structured types:

```rust
use kraky::{KrakyError, KrakenApiError};

// Kraken returns errors like "EQuery:Unknown asset pair"
// The SDK parses these into structured errors:

match result {
    Err(KrakyError::KrakenApi(e)) => {
        println!("Severity: {}", e.severity);   // Error
        println!("Category: {}", e.category);   // Query
        println!("Message: {}", e.message);     // Unknown asset pair
        
        if e.is_retryable() {
            // Retry logic for temporary errors
        }
        if e.is_rate_limited() {
            // Back off
        }
        if e.is_invalid_pair() {
            // Fix pair name
        }
    }
    Err(KrakyError::RateLimited) => {
        // Auto-mapped from "EAPI:Rate limit exceeded"
    }
    Err(KrakyError::InvalidPair(msg)) => {
        // Auto-mapped from "EQuery:Unknown asset pair"
    }
    _ => {}
}
```

### Connection Errors

```rust
use kraky::{KrakyClient, KrakyError};

match KrakyClient::connect().await {
    Ok(client) => { /* success */ }
    Err(KrakyError::Connection(e)) => { /* WebSocket error */ }
    Err(KrakyError::Json(e)) => { /* Parsing error */ }
    Err(e) => { /* Other errors */ }
}
```

## Data Types

### Orderbook

```rust
pub struct Orderbook {
    pub symbol: String,
    pub bids: BTreeMap<OrderedFloat, f64>,  // Price -> Quantity
    pub asks: BTreeMap<OrderedFloat, f64>,
    pub timestamp: String,
    pub sequence: u64,
}

impl Orderbook {
    // Price methods
    pub fn best_bid(&self) -> Option<f64>;
    pub fn best_ask(&self) -> Option<f64>;
    pub fn spread(&self) -> Option<f64>;
    pub fn mid_price(&self) -> Option<f64>;
    pub fn top_bids(&self, n: usize) -> Vec<PriceLevel>;
    pub fn top_asks(&self, n: usize) -> Vec<PriceLevel>;
    
    // Imbalance detection
    pub fn imbalance(&self) -> f64;                           // Full book (-1.0 to 1.0)
    pub fn imbalance_top_n(&self, n: usize) -> f64;           // Top N levels only
    pub fn imbalance_within_depth(&self, pct: f64) -> Option<f64>; // Within % of mid
    pub fn imbalance_metrics(&self) -> ImbalanceMetrics;      // Full metrics
}
```

### Trade

```rust
pub struct Trade {
    pub symbol: String,
    pub side: TradeSide,      // Buy or Sell
    pub price: f64,
    pub qty: f64,
    pub ord_type: OrderType,  // Market or Limit
    pub trade_id: i64,
    pub timestamp: String,
}
```

### Ticker

```rust
pub struct Ticker {
    pub symbol: String,
    pub bid: f64,
    pub bid_qty: f64,
    pub ask: f64,
    pub ask_qty: f64,
    pub last: f64,
    pub volume: f64,
    pub vwap: f64,
    pub low: f64,
    pub high: f64,
    pub change: f64,
    pub change_pct: f64,
}
```

### OHLC

```rust
pub struct OHLC {
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub vwap: f64,
    pub volume: f64,
    pub count: i64,
    pub interval: u32,
    pub timestamp: String,
}
```

## Orderbook Imbalance Detection

The SDK provides built-in orderbook imbalance calculations for detecting buy/sell pressure:

```rust
if let Some(ob) = client.get_orderbook("BTC/USD") {
    // Simple imbalance (-1.0 to 1.0)
    // Positive = more bids (bullish), Negative = more asks (bearish)
    let imbalance = ob.imbalance();
    println!("Imbalance: {:+.2}%", imbalance * 100.0);
    
    // Top-of-book imbalance (most actionable for trading)
    let top5_imbalance = ob.imbalance_top_n(5);
    
    // Imbalance within 0.5% of mid price
    if let Some(tight) = ob.imbalance_within_depth(0.005) {
        println!("Tight spread imbalance: {:+.2}%", tight * 100.0);
    }
    
    // Full metrics with signals
    let metrics = ob.imbalance_metrics();
    println!("Bid Volume: {:.4} BTC", metrics.bid_volume);
    println!("Ask Volume: {:.4} BTC", metrics.ask_volume);
    println!("Bid/Ask Ratio: {:.2}", metrics.bid_ask_ratio);
    
    // Generate trading signals
    match metrics.signal(0.1) {  // 10% threshold
        ImbalanceSignal::Bullish => println!("🟢 BULLISH - more buy pressure"),
        ImbalanceSignal::Bearish => println!("🔴 BEARISH - more sell pressure"),
        ImbalanceSignal::Neutral => println!("⚪ NEUTRAL - balanced"),
    }
}
```

### Imbalance Metrics

| Method | Description |
|--------|-------------|
| `imbalance()` | Full orderbook imbalance (-1.0 to 1.0) |
| `imbalance_top_n(n)` | Imbalance of top N levels only |
| `imbalance_within_depth(pct)` | Imbalance within % of mid price |
| `imbalance_metrics()` | Detailed metrics (volumes, ratio, signal) |

## 🤖 Telegram Bot Integration

Get real-time market alerts delivered to Telegram using Kraky's advanced orderbook analytics. This feature showcases a practical, real-world application of the SDK's imbalance detection capabilities.

### Features

- 📊 **Price Alerts** - Get notified when price crosses thresholds
- 🎯 **Imbalance Signals** - Bullish/Bearish/Neutral signals based on orderbook depth
- 📈 **Orderbook Summaries** - Best bid/ask, spread, mid-price updates
- 🔔 **Connection Events** - Monitor WebSocket connection status
- ⚡ **Real-time Delivery** - Instant notifications via Telegram

### Installation

The Telegram integration is **optional** and adds approximately 800KB to your binary size.

```toml
# Enable Telegram alerts with imbalance detection
kraky = { git = "https://github.com/SarpTekin/kraky", features = ["telegram-alerts"] }

# Or just basic Telegram notifications
kraky = { git = "https://github.com/SarpTekin/kraky", features = ["telegram"] }
```

**Note:** Remains lightweight - users who don't enable this feature add 0 bytes to their binary.

### Quick Start

```rust
use kraky::{KrakyClient, TelegramNotifier, ImbalanceSignal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Kraky client
    let client = KrakyClient::connect().await?;

    // Create Telegram bot
    let bot = TelegramNotifier::new(
        "YOUR_BOT_TOKEN",
        123456789  // Your chat ID
    );

    // Subscribe to orderbook
    let mut orderbook = client.subscribe_orderbook("BTC/USD", 10).await?;

    // Send startup notification
    bot.send_connection_status(true, "Bot started!").await?;

    // Monitor for imbalance signals
    while let Some(_) = orderbook.next().await {
        if let Some(ob) = client.get_orderbook("BTC/USD") {
            let metrics = ob.imbalance_metrics();
            let signal = metrics.signal(0.15); // 15% threshold

            // Send alert when signal changes
            if !matches!(signal, ImbalanceSignal::Neutral) {
                bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
            }
        }
    }

    Ok(())
}
```

### Setup Instructions

1. **Create a Telegram Bot**
   - Message [@BotFather](https://t.me/BotFather) on Telegram
   - Send `/newbot` and follow the instructions
   - Save your bot token

2. **Get Your Chat ID**
   - Message [@userinfobot](https://t.me/userinfobot)
   - It will reply with your chat ID

3. **Set Environment Variables**
   ```bash
   export TELEGRAM_BOT_TOKEN="123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
   export TELEGRAM_CHAT_ID="987654321"
   ```

4. **Run the Example**
   ```bash
   cargo run --example telegram_imbalance_bot --features telegram-alerts
   ```

### Alert Types

#### Imbalance Alerts (Leverages Kraky's Unique Analytics)

The most powerful feature - get notified of orderbook pressure changes:

```rust
let metrics = ob.imbalance_metrics();
let signal = metrics.signal(0.15); // 15% imbalance threshold

bot.send_imbalance_alert("BTC/USD", &metrics, signal).await?;
```

**Example Telegram Message:**
```
🟢 BTC/USD Orderbook Imbalance Alert

📊 Signal: BULLISH
──────────────────────────────

📈 Metrics:
• Bid Volume: 12.3456 BTC
• Ask Volume: 8.7654 BTC
• Bid/Ask Ratio: 1.41
• Imbalance: +17.02%

💡 Interpretation:
Strong buy pressure detected - more bids than asks
```

#### Price Threshold Alerts

```rust
bot.send_threshold_alert("BTC/USD", 100500.0, 100000.0, true).await?;
```

**Example Message:**
```
📈 BTC/USD Threshold Alert

Current Price: $100,500.00
Threshold: $100,000.00
Status: Price is above threshold
Change: 0.50%
```

#### Orderbook Summary

```rust
bot.send_orderbook_summary(
    "BTC/USD",
    99500.0,  // best bid
    99505.0,  // best ask
    5.0,      // spread
    99502.5   // mid price
).await?;
```

#### Connection Status

```rust
bot.send_connection_status(true, "Connected to Kraken WebSocket").await?;
```

### Example: Complete Trading Bot

See `examples/telegram_imbalance_bot.rs` for a complete implementation featuring:
- Real-time imbalance monitoring
- Price threshold alerts
- Connection event notifications
- Hourly summary reports
- Configurable alert thresholds

```bash
cargo run --example telegram_imbalance_bot --features telegram-alerts
```

### Why This Matters

This Telegram integration demonstrates:

1. **Practical Application** - Real-world use case for market data
2. **Modular Design** - Optional feature that doesn't bloat the core SDK
3. **Advanced Analytics** - Leverages Kraky's superior imbalance detection
4. **User-Friendly** - Easy setup with environment variables
5. **Production-Ready** - Async, error handling, connection monitoring

### API Reference

#### Public Market Data Notifications

| Method | Description |
|--------|-------------|
| `send_alert(message)` | Send basic text alert |
| `send_imbalance_alert(symbol, metrics, signal)` | Send imbalance signal with full metrics |
| `send_price_alert(symbol, price, context)` | Send formatted price alert |
| `send_threshold_alert(symbol, price, threshold, above)` | Send threshold crossing alert |
| `send_orderbook_summary(symbol, bid, ask, spread, mid)` | Send orderbook snapshot |
| `send_connection_status(connected, details)` | Send connection status update |
| `send_whale_alert(symbol, side, price, volume)` | Send large order detection alert |
| `send_spread_alert(symbol, spread_bps, avg_spread, ratio)` | Send wide spread warning |
| `send_divergence_alert(symbol, price1, price2, pct_diff)` | Send price divergence alert |
| `send_trade_alert(symbol, side, price, volume)` | Send significant trade alert |

#### Private Account Notifications (requires `private` feature)

| Method | Description |
|--------|-------------|
| `send_balance_update(update)` | Send account balance change notification |
| `send_order_update(update)` | Send order status update (placed, filled, cancelled) |
| `send_execution_alert(update)` | Send trade execution notification |
| `send_portfolio_summary(update)` | Send portfolio summary with total value |

#### Trading Notifications (requires `trading` feature)

| Method | Description |
|--------|-------------|
| `send_order_placed(response, params)` | Send order placement confirmation |
| `send_order_filled(symbol, side, qty, price, id)` | Send order fill notification |
| `send_order_cancelled(symbol, order_id, reason)` | Send order cancellation alert |
| `send_order_failed(params, error)` | Send order failure notification |
| `send_order_amended(response, params)` | Send order modification confirmation |
| `send_trading_summary(trades, volume, pl, win_rate)` | Send daily trading summary |

## Orderbook Checksum Validation

Kraken sends CRC32 checksums with orderbook updates. The SDK validates these automatically:

```rust
if let Some(ob) = client.get_orderbook("BTC/USD") {
    // Check if last update had valid checksum
    if !ob.checksum_valid {
        println!("Orderbook might be corrupted!");
        client.reconnect()?; // Get fresh snapshot
    }

    // Calculate and display checksum
    let checksum = ob.calculate_checksum();
    println!("Checksum: 0x{:08X}", checksum);
}

// Validate all orderbooks and auto-reconnect if corrupted
let corrupted_count = client.validate_orderbooks_and_reconnect()?;

// Quick check for specific pair
if client.is_orderbook_valid("BTC/USD") == Some(false) {
    client.reconnect()?;
}
```

### Checksum Methods

| Method | Description |
|--------|-------------|
| `calculate_checksum()` | Calculate CRC32 of top 10 levels |
| `validate_checksum(expected)` | Returns `true` if checksum matches |
| `checksum_validation(expected)` | Returns detailed `ChecksumValidation` struct |

## Smart Reconnection

The SDK automatically reconnects when the connection drops, with configurable exponential backoff:

```rust
use kraky::{KrakyClient, ReconnectConfig};

// Default: automatic reconnection with exponential backoff
let client = KrakyClient::connect().await?;

// Aggressive reconnection (for low-latency needs)
let client = KrakyClient::connect_with_reconnect(ReconnectConfig::aggressive()).await?;

// Conservative reconnection (to avoid rate limiting)
let client = KrakyClient::connect_with_reconnect(ReconnectConfig::conservative()).await?;

// Disable reconnection
let client = KrakyClient::connect_with_reconnect(ReconnectConfig::disabled()).await?;

// Custom configuration
let config = ReconnectConfig {
    enabled: true,
    initial_delay: Duration::from_millis(200),
    max_delay: Duration::from_secs(10),
    backoff_multiplier: 1.5,
    max_attempts: Some(20),
};
let client = KrakyClient::connect_with_reconnect(config).await?;
```

### Connection State

Monitor connection state programmatically:

```rust
use kraky::ConnectionState;

// Check connection status
if client.is_connected() {
    println!("Connected!");
}

if client.is_reconnecting() {
    println!("Reconnecting...");
}

// Get detailed state
match client.connection_state() {
    ConnectionState::Connected => println!("Ready"),
    ConnectionState::Reconnecting => println!("Reconnecting..."),
    ConnectionState::Connecting => println!("Initial connection..."),
    ConnectionState::Disconnected => println!("Disconnected"),
}

// Manually trigger reconnection
client.reconnect()?;
```

### Reconnect Presets

| Preset | Initial Delay | Max Delay | Backoff | Max Attempts |
|--------|---------------|-----------|---------|--------------|
| `default()` | 500ms | 30s | 2.0x | Unlimited |
| `aggressive()` | 100ms | 5s | 1.5x | Unlimited |
| `conservative()` | 1s | 60s | 2.0x | 10 |
| `disabled()` | - | - | - | 0 |

### Connection Events

Subscribe to connection lifecycle events for monitoring and logging:

```rust
use kraky::ConnectionEvent;

let mut events = client.subscribe_events();

tokio::spawn(async move {
    while let Some(event) = events.recv().await {
        match event {
            ConnectionEvent::Connected => println!("✅ Connected"),
            ConnectionEvent::Disconnected(reason) => println!("❌ Disconnected: {:?}", reason),
            ConnectionEvent::Reconnecting(n) => println!("🔄 Reconnecting (attempt #{})", n),
            ConnectionEvent::Reconnected => println!("✅ Reconnected"),
            ConnectionEvent::ReconnectFailed(n, e) => println!("⚠️ Attempt #{} failed: {}", n, e),
            ConnectionEvent::ReconnectExhausted => println!("💀 Max attempts reached"),
        }
    }
});
```

| Event | Description |
|-------|-------------|
| `Connected` | Initial connection successful |
| `Disconnected(reason)` | Connection lost |
| `Reconnecting(attempt)` | Starting reconnection attempt |
| `Reconnected` | Reconnection successful |
| `ReconnectFailed(attempt, error)` | Reconnection attempt failed |
| `ReconnectExhausted` | Max attempts reached, giving up |

## Backpressure Monitoring

Subscriptions use bounded channels (default: 1000 messages). If your consumer is too slow, older messages are dropped to keep the latest data:

```rust
let mut trades = client.subscribe_trades("BTC/USD").await?;

// Process messages...
while let Some(trade) = trades.next().await {
    // Your processing logic
}

// Check stats
let stats = trades.stats();
println!("Delivered: {}, Dropped: {}, Drop rate: {:.2}%", 
    stats.delivered(), 
    stats.dropped(), 
    stats.drop_rate()
);
```

## Supported Trading Pairs

The SDK supports **all trading pairs** available on Kraken. Common pairs include:

- `BTC/USD`, `BTC/EUR`, `BTC/USDT`
- `ETH/USD`, `ETH/EUR`, `ETH/BTC`
- `XRP/USD`, `SOL/USD`, `DOT/USD`
- `DOGE/USD`, `ADA/USD`, `AVAX/USD`

See [Kraken's asset pairs](https://support.kraken.com/hc/en-us/articles/201893658-Currency-pairs-available-for-trading-on-Kraken) for the full list.

## Performance

The SDK is designed for low-latency market data processing:

| Feature | Benefit |
|---------|---------|
| Async I/O (Tokio) | Non-blocking network operations |
| Zero-copy parsing | Efficient memory usage |
| Managed state | Pre-computed orderbook metrics |
| Bounded channels | Backpressure prevents memory issues |
| Structured errors | Fast error categorization |

## Test Coverage

```bash
cargo test
```

**29 tests** covering:
- Orderbook operations (17 tests) - including imbalance & checksum validation
- Subscription handling (4 tests)
- Error parsing (6 tests)
- Reconnection logic (2 tests)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This SDK is provided for educational and informational purposes. Trading cryptocurrencies involves risk. Use at your own discretion.

---

Built for the [Kraken Forge Hackathon](https://krakenforgechallenge.devpost.com/) 🐙
