# Kraky

A lightweight, high-performance Rust SDK for connecting to the [Kraken Exchange](https://www.kraken.com/) WebSocket API v2.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

## Features

- **Real-time Market Data**: Stream orderbook updates, trades, tickers, and OHLC candles
- **Managed Orderbook State**: Automatic orderbook reconstruction from incremental updates
- **Type-safe API**: Strongly typed data models for all Kraken message types
- **Async/Await**: Built on Tokio for efficient async I/O
- **Zero-copy Parsing**: Efficient JSON deserialization with Serde
- **Automatic Reconnection**: Built-in heartbeat handling
- **Backpressure Control**: Bounded channels prevent memory issues with slow consumers

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kraky = "0.1.0"
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
use futures_util::StreamExt;

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

## Error Handling

The SDK uses a custom error type for comprehensive error handling:

```rust
use kraky::{KrakyClient, KrakyError};

match KrakyClient::connect().await {
    Ok(client) => { /* success */ }
    Err(KrakyError::Connection(e)) => { /* WebSocket error */ }
    Err(KrakyError::Json(e)) => { /* Parsing error */ }
    Err(e) => { /* Other errors */ }
}
```

## Examples

Run the included examples:

```bash
# Orderbook depth visualization
cargo run --example orderbook

# Real-time trades
cargo run --example trades

# Ticker updates
cargo run --example ticker

# OHLC candles
cargo run --example ohlc

# Multiple subscriptions
cargo run --example multi_subscribe
```

## Supported Trading Pairs

The SDK supports all trading pairs available on Kraken. Common pairs include:

- `BTC/USD`, `BTC/EUR`
- `ETH/USD`, `ETH/EUR`
- `XRP/USD`, `SOL/USD`
- `DOGE/USD`, `ADA/USD`

See [Kraken's asset pairs](https://support.kraken.com/hc/en-us/articles/201893658-Currency-pairs-available-for-trading-on-Kraken) for the full list.

## Performance

The SDK is designed for low-latency market data processing:

- **Zero-copy where possible**: Efficient memory usage
- **Async I/O**: Non-blocking network operations
- **Managed state**: Pre-computed orderbook metrics
- **Backpressure control**: Bounded channels prevent memory issues

### Backpressure Monitoring

Subscriptions use bounded channels (default: 1000 messages). If your consumer is too slow, older messages are dropped to keep the latest data. Monitor backpressure with:

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

## Documentation

- **[Developer Guide](docs/DEVELOPER_GUIDE.md)** - Complete tutorial with examples
- **[Architecture](ARCHITECTURE.md)** - Internal design and code walkthrough

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This SDK is provided for educational and informational purposes. Trading cryptocurrencies involves risk. Use at your own discretion.

---

Built for the [Kraken Forge Hackathon](https://krakenforgechallenge.devpost.com/) 🐙

