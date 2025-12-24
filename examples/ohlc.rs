//! OHLC (Candlestick) subscription example
//!
//! This example demonstrates how to subscribe to OHLC/candlestick data
//! for technical analysis.

use kraky::{Interval, KrakyClient};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("kraky=info".parse()?))
        .init();

    println!("Connecting to Kraken WebSocket...");

    let client = KrakyClient::connect().await?;

    println!("Connected! Subscribing to BTC/USD 1-minute candles...");

    let mut subscription = client.subscribe_ohlc("BTC/USD", Interval::Min1).await?;

    println!("Subscribed! Waiting for OHLC updates...\n");
    println!(
        "{:<20} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "TIME", "OPEN", "HIGH", "LOW", "CLOSE", "VOLUME"
    );
    println!("{}", "─".repeat(90));

    let mut count = 0;
    while let Some(ohlc) = subscription.next().await {
        println!(
            "{:<20} {:>12.2} {:>12.2} {:>12.2} {:>12.2} {:>12.6}",
            &ohlc.timestamp[..19],
            ohlc.open,
            ohlc.high,
            ohlc.low,
            ohlc.close,
            ohlc.volume
        );

        count += 1;
        if count >= 10 {
            println!("\nReceived 10 updates, disconnecting...");
            break;
        }
    }

    client.disconnect();
    println!("Done!");

    Ok(())
}
