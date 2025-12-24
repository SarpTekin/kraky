//! Ticker subscription example
//!
//! This example demonstrates how to subscribe to ticker updates
//! for price and volume information.

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

    println!("Connected! Subscribing to BTC/USD ticker...");

    let mut subscription = client.subscribe_ticker("BTC/USD").await?;

    println!("Subscribed! Waiting for ticker updates...\n");

    let mut count = 0;
    while let Some(ticker) = subscription.next().await {
        println!("═══════════════════════════════════════════════════");
        println!("  {} Ticker", ticker.symbol);
        println!("═══════════════════════════════════════════════════");
        println!();
        println!("  Last Price:    ${:.2}", ticker.last);
        println!(
            "  Bid:           ${:.2} ({:.4})",
            ticker.bid, ticker.bid_qty
        );
        println!(
            "  Ask:           ${:.2} ({:.4})",
            ticker.ask, ticker.ask_qty
        );
        println!("  24h Change:    {:.2}%", ticker.change_pct);
        println!("  24h Volume:    {:.4} BTC", ticker.volume);
        println!("  24h VWAP:      ${:.2}", ticker.vwap);
        println!("  24h High:      ${:.2}", ticker.high);
        println!("  24h Low:       ${:.2}", ticker.low);
        println!();

        count += 1;
        if count >= 5 {
            println!("Received 5 updates, disconnecting...");
            break;
        }
    }

    client.disconnect();
    println!("Done!");

    Ok(())
}
