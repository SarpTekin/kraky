//! Orderbook subscription example
//!
//! This example demonstrates how to subscribe to orderbook updates
//! and maintain a local orderbook state.

use kraky::KrakyClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("kraky=info".parse()?))
        .init();

    println!("Connecting to Kraken WebSocket...");

    // Connect to Kraken
    let client = KrakyClient::connect().await?;

    println!("Connected! Subscribing to BTC/USD orderbook...");

    // Subscribe to orderbook with depth of 10 levels
    let mut subscription = client.subscribe_orderbook("BTC/USD", 10).await?;

    println!("Subscribed! Waiting for updates...\n");

    // Process updates
    let mut count = 0;
    while let Some(update) = subscription.next().await {
        for data in &update.data {
            println!("═══════════════════════════════════════════════════");
            println!(
                "  {} Orderbook Update ({})",
                data.symbol, update.update_type
            );
            println!("═══════════════════════════════════════════════════");

            // Get current orderbook state
            if let Some(orderbook) = client.get_orderbook(&data.symbol) {
                let top_bids = orderbook.top_bids(5);
                let top_asks = orderbook.top_asks(5);

                println!(
                    "\n  {:>12}  {:>12}  │  {:>12}  {:>12}",
                    "Bid Qty", "Bid Price", "Ask Price", "Ask Qty"
                );
                println!("  ─────────────────────────────┼─────────────────────────────");

                for i in 0..5 {
                    let bid = top_bids.get(i);
                    let ask = top_asks.get(i);

                    let bid_qty = bid.map(|b| format!("{:.4}", b.qty)).unwrap_or_default();
                    let bid_price = bid.map(|b| format!("{:.2}", b.price)).unwrap_or_default();
                    let ask_price = ask.map(|a| format!("{:.2}", a.price)).unwrap_or_default();
                    let ask_qty = ask.map(|a| format!("{:.4}", a.qty)).unwrap_or_default();

                    println!(
                        "  {:>12}  {:>12}  │  {:>12}  {:>12}",
                        bid_qty, bid_price, ask_price, ask_qty
                    );
                }

                if let (Some(spread), Some(mid)) = (orderbook.spread(), orderbook.mid_price()) {
                    println!("\n  Spread: ${:.2} | Mid: ${:.2}", spread, mid);
                }
            }

            println!();
        }

        count += 1;
        if count >= 10 {
            println!("Received 10 updates, disconnecting...");
            break;
        }
    }

    client.disconnect();
    println!("Done!");

    Ok(())
}
