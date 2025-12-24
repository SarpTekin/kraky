//! Multi-subscription example
//!
//! This example demonstrates how to subscribe to multiple data streams
//! simultaneously and process them concurrently.

use kraky::KrakyClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("kraky=info".parse()?))
        .init();

    println!("Connecting to Kraken WebSocket...");

    let client = KrakyClient::connect().await?;

    println!("Connected! Subscribing to multiple streams...\n");

    // Subscribe to multiple pairs
    let mut btc_trades = client.subscribe_trades("BTC/USD").await?;
    let mut eth_trades = client.subscribe_trades("ETH/USD").await?;
    let mut btc_ticker = client.subscribe_ticker("BTC/USD").await?;

    println!("Subscribed to:");
    println!("  - BTC/USD trades");
    println!("  - ETH/USD trades");
    println!("  - BTC/USD ticker");
    println!("\nWaiting for updates...\n");

    let mut count = 0;

    loop {
        tokio::select! {
            Some(trade) = btc_trades.next() => {
                println!("[BTC TRADE] {} {:.6} @ ${:.2}",
                    trade.side, trade.qty, trade.price);
                count += 1;
            }
            Some(trade) = eth_trades.next() => {
                println!("[ETH TRADE] {} {:.6} @ ${:.2}",
                    trade.side, trade.qty, trade.price);
                count += 1;
            }
            Some(ticker) = btc_ticker.next() => {
                println!("[BTC TICKER] Last: ${:.2} | Bid: ${:.2} | Ask: ${:.2} | 24h: {:.2}%",
                    ticker.last, ticker.bid, ticker.ask, ticker.change_pct);
                count += 1;
            }
        }

        if count >= 20 {
            println!("\nReceived 20 updates, disconnecting...");
            break;
        }
    }

    client.disconnect();
    println!("Done!");

    Ok(())
}
