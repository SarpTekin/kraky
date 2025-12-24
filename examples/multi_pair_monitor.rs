//! 📊 Multi-Pair Monitor
//!
//! Monitors multiple trading pairs simultaneously and displays real-time updates.
//! Demonstrates concurrent subscriptions and efficient data handling.
//!
//! ## Features
//! - Monitor 3+ pairs simultaneously
//! - Real-time price updates
//! - Orderbook imbalance signals
//! - Spread monitoring
//! - Clean terminal UI
//!
//! ## Run
//! ```bash
//! cargo run --example multi_pair_monitor --features analytics
//! ```

use kraky::KrakyClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging (quieter for cleaner output)
    tracing_subscriber::fmt()
        .with_env_filter("warn")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           📊 Multi-Pair Monitor - Demo                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Trading pairs to monitor
    let pairs = vec!["BTC/USD", "ETH/USD", "SOL/USD"];

    println!("⚙️  Configuration:");
    println!("   Monitoring: {}", pairs.join(", "));
    println!("   Features: Price, Spread, Imbalance\n");

    // Connect to Kraken
    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // Subscribe to all pairs
    println!("📊 Subscribing to orderbooks...");

    for pair in &pairs {
        client.subscribe_orderbook(pair, 10).await?;
        println!("   ✅ Subscribed to {}", pair);
    }
    println!();

    // Track state for each pair
    let mut pair_state: HashMap<String, PairState> = HashMap::new();
    for pair in &pairs {
        pair_state.insert(pair.to_string(), PairState::default());
    }

    println!("🚀 Monitor is now active!");
    println!("   Press Ctrl+C to stop\n");
    println!("{}", "═".repeat(80));

    let mut update_count = 0;

    // Monitor all pairs concurrently using a simple polling pattern
    loop {
        // Update state from current orderbooks and tickers
        for pair in &pairs {
            if let Some(ob) = client.get_orderbook(pair) {
                let state = pair_state.get_mut(*pair).unwrap();
                state.orderbook_updates += 1;

                // Get best bid/ask
                if let (Some(best_bid), Some(best_ask)) = (ob.best_bid(), ob.best_ask()) {
                    state.best_bid = best_bid;
                    state.best_ask = best_ask;
                    state.spread = best_ask - best_bid;

                    // Calculate imbalance
                    #[cfg(feature = "analytics")]
                    {
                        let metrics = ob.imbalance_metrics();
                        state.imbalance = metrics.imbalance_ratio;
                        state.signal = Some(metrics.signal(0.15));
                    }
                }
            }
        }

        // Display update every 50 iterations
        update_count += 1;
        if update_count % 50 == 0 {
            display_status(&pairs, &pair_state);
        }

        // Small delay to avoid busy loop
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

#[derive(Default)]
struct PairState {
    orderbook_updates: u64,
    best_bid: f64,
    best_ask: f64,
    spread: f64,
    imbalance: f64,
    #[cfg(feature = "analytics")]
    signal: Option<kraky::ImbalanceSignal>,
}

fn display_status(pairs: &[&str], state: &HashMap<String, PairState>) {
    println!("\n{}", "═".repeat(80));
    println!("{:^80}", "📊 MULTI-PAIR MONITOR STATUS");
    println!("{}", "═".repeat(80));

    for pair in pairs {
        if let Some(s) = state.get(*pair) {
            let mid_price = (s.best_bid + s.best_ask) / 2.0;
            let spread_bps = if mid_price > 0.0 {
                (s.spread / mid_price) * 10000.0
            } else {
                0.0
            };

            #[cfg(feature = "analytics")]
            let signal_str = match s.signal {
                Some(kraky::ImbalanceSignal::Bullish) => "🟢 BULLISH",
                Some(kraky::ImbalanceSignal::Bearish) => "🔴 BEARISH",
                Some(kraky::ImbalanceSignal::Neutral) => "⚪ NEUTRAL",
                None => "⚪ N/A",
            };

            #[cfg(not(feature = "analytics"))]
            let signal_str = "N/A";

            println!("\n📈 {}", pair);
            println!("   Mid Price: ${:.2}", mid_price);
            println!("   Best Bid/Ask: ${:.2} / ${:.2}", s.best_bid, s.best_ask);
            println!("   Spread: ${:.2} ({:.1} bps)", s.spread, spread_bps);
            println!("   Imbalance: {:+.2}% | Signal: {}", s.imbalance * 100.0, signal_str);
            println!("   Orderbook Updates: {}", s.orderbook_updates);
        }
    }

    println!("\n{}", "═".repeat(80));
}
