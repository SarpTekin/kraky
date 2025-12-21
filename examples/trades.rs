//! Trades subscription example
//!
//! This example demonstrates how to subscribe to real-time trade data.

use kraky::{KrakyClient, TradeSide};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("kraky=info".parse()?))
        .init();

    println!("Connecting to Kraken WebSocket...");
    
    let client = KrakyClient::connect().await?;
    
    println!("Connected! Subscribing to BTC/USD trades...");
    
    let mut subscription = client.subscribe_trades("BTC/USD").await?;
    
    println!("Subscribed! Waiting for trades...\n");
    println!("{:<20} {:>10} {:>12} {:>12}", "TIME", "SIDE", "PRICE", "QUANTITY");
    println!("{}", "─".repeat(60));
    
    let mut count = 0;
    while let Some(trade) = subscription.next().await {
        let side_indicator = match trade.side {
            TradeSide::Buy => "🟢 BUY",
            TradeSide::Sell => "🔴 SELL",
        };
        
        println!("{:<20} {:>10} {:>12.2} {:>12.6}",
            &trade.timestamp[..19],
            side_indicator,
            trade.price,
            trade.qty
        );
        
        count += 1;
        if count >= 20 {
            println!("\nReceived 20 trades, disconnecting...");
            break;
        }
    }
    
    client.disconnect();
    println!("Done!");
    
    Ok(())
}

