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

# Run all tests (19 tests)
cargo test

# ⭐ RECOMMENDED: Run the comprehensive demo (shows all features)
cargo run --example demo

# Or run individual examples:
cargo run --example orderbook       # Live orderbook depth
cargo run --example trades          # Real-time trades
cargo run --example ticker          # Ticker updates
cargo run --example ohlc            # OHLC candles
cargo run --example multi_subscribe # Multiple subscriptions
```

### What You'll See

**Demo Example Output:**
```
╔══════════════════════════════════════════════════════════════╗
║           🐙 KRAKY SDK DEMO - Kraken Forge Hackathon          ║
╚══════════════════════════════════════════════════════════════╝

📡 Connecting to Kraken WebSocket API...
✅ Connected!

📖 ORDERBOOK UPDATE #1
   Best Bid: $97234.50
   Best Ask: $97235.00
   Spread:   $0.50
   Mid:      $97234.75
   Top 3 Bids: ["$97234", "$97233", "$97232"]
   Top 3 Asks: ["$97235", "$97236", "$97237"]

🟢 TRADE: Buy 0.050000 BTC @ $97235.00
🔴 TRADE: Sell 0.120000 BTC @ $97234.50
📈 TICKER: $97235.00 (24h: +2.35%) Vol: 1234.56 BTC

═══════════════════════════════════════════════════════════════
                    DEMO STATISTICS
═══════════════════════════════════════════════════════════════
Messages received in 15 seconds:
   📖 Orderbook updates: 47
   💱 Trades:            23
   📈 Ticker updates:    12

Backpressure stats (delivered / dropped / drop rate):
   📖 Orderbook: 47 / 0 / 0.00%
   💱 Trades:    23 / 0 / 0.00%
   📈 Ticker:    12 / 0 / 0.00%

╔══════════════════════════════════════════════════════════════╗
║                    🎉 DEMO COMPLETE!                          ║
╚══════════════════════════════════════════════════════════════╝
```

### Key Features Demonstrated

| Example | What It Shows |
|---------|---------------|
| ⭐ `demo` | **All features in one place** - orderbook, trades, ticker, backpressure stats |
| `orderbook` | Real-time depth, managed state, spread calculation |
| `trades` | Live trade stream, buy/sell sides |
| `ticker` | Price, volume, 24h change |
| `ohlc` | Candlestick data for charting |
| `multi_subscribe` | Concurrent subscriptions with `tokio::select!` |

---

## Features

- **Real-time Market Data**: Stream orderbook, trades, tickers, and OHLC candles
- **Managed Orderbook State**: Automatic reconstruction from incremental updates
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
    pub fn best_bid(&self) -> Option<f64>;
    pub fn best_ask(&self) -> Option<f64>;
    pub fn spread(&self) -> Option<f64>;
    pub fn mid_price(&self) -> Option<f64>;
    pub fn top_bids(&self, n: usize) -> Vec<PriceLevel>;
    pub fn top_asks(&self, n: usize) -> Vec<PriceLevel>;
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

**19 tests** covering:
- Orderbook operations (9 tests)
- Subscription handling (4 tests)
- Error parsing (6 tests)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This SDK is provided for educational and informational purposes. Trading cryptocurrencies involves risk. Use at your own discretion.

---

Built for the [Kraken Forge Hackathon](https://krakenforgechallenge.devpost.com/) 🐙
