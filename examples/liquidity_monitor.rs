//! 💧 Liquidity Monitor
//!
//! Monitors market liquidity by tracking orderbook depth and spread.
//! Alerts when liquidity drops below thresholds (indicating potential volatility).
//!
//! ## Features
//! - Track total bid/ask volume
//! - Monitor spread changes
//! - Detect liquidity dry-ups
//! - Telegram alerts on low liquidity
//!
//! ## Run
//! ```bash
//! # Without Telegram
//! cargo run --example liquidity_monitor --features analytics
//!
//! # With Telegram alerts
//! export TELEGRAM_BOT_TOKEN="your_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! cargo run --example liquidity_monitor --features telegram-alerts
//! ```

use kraky::KrakyClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           💧 Liquidity Monitor - Demo                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Configuration
    let trading_pair = "BTC/USD";
    let min_total_liquidity = 50.0; // BTC
    let max_spread_bps = 50.0; // basis points
    let check_interval = Duration::from_secs(10);

    println!("⚙️  Configuration:");
    println!("   Trading Pair: {}", trading_pair);
    println!("   Min Total Liquidity: {} BTC", min_total_liquidity);
    println!("   Max Spread: {} bps", max_spread_bps);
    println!("   Check Interval: {:?}\n", check_interval);

    // Telegram setup (optional)
    #[cfg(feature = "telegram")]
    let bot = {
        match (
            std::env::var("TELEGRAM_BOT_TOKEN"),
            std::env::var("TELEGRAM_CHAT_ID"),
        ) {
            (Ok(token), Ok(chat_id)) => {
                let id: i64 = chat_id.parse().expect("Invalid TELEGRAM_CHAT_ID");
                let notifier = kraky::TelegramNotifier::new(&token, id);
                println!("📱 Telegram alerts: ENABLED\n");
                Some(notifier)
            }
            _ => {
                println!("📱 Telegram alerts: DISABLED (set env vars to enable)\n");
                None
            }
        }
    };

    // Connect to Kraken
    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // Subscribe to orderbook
    println!(
        "📊 Subscribing to {} orderbook (depth: 100)...",
        trading_pair
    );
    let mut orderbook_sub = client.subscribe_orderbook(trading_pair, 100).await?;
    println!("✅ Subscribed\n");

    println!("🚀 Liquidity Monitor is now active!");
    println!("   Press Ctrl+C to stop\n");
    println!("{}", "═".repeat(70));

    // Track state
    let mut update_count = 0;
    let mut last_check = std::time::Instant::now();
    let mut spread_history: Vec<f64> = Vec::new();

    // Main loop
    while let Some(_update) = orderbook_sub.next().await {
        update_count += 1;

        if last_check.elapsed() >= check_interval {
            if let Some(ob) = client.get_orderbook(trading_pair) {
                // Calculate total liquidity
                let total_bid_volume: f64 = ob.bids.values().sum();
                let total_ask_volume: f64 = ob.asks.values().sum();
                let total_liquidity = total_bid_volume + total_ask_volume;

                // Calculate spread
                if let (Some(best_bid), Some(best_ask)) = (ob.best_bid(), ob.best_ask()) {
                    let spread = best_ask - best_bid;
                    let mid_price = (best_bid + best_ask) / 2.0;
                    let spread_bps = (spread / mid_price) * 10000.0;

                    // Track spread history
                    spread_history.push(spread_bps);
                    if spread_history.len() > 100 {
                        spread_history.remove(0);
                    }

                    let avg_spread = if !spread_history.is_empty() {
                        spread_history.iter().sum::<f64>() / spread_history.len() as f64
                    } else {
                        spread_bps
                    };

                    // Display status
                    println!("\n💧 Liquidity Status (Update #{})", update_count);
                    println!("{}", "─".repeat(70));
                    println!("   Total Liquidity: {:.2} BTC", total_liquidity);
                    println!("   Bid Volume: {:.2} BTC", total_bid_volume);
                    println!("   Ask Volume: {:.2} BTC", total_ask_volume);
                    println!("   Best Bid: ${:.2}", best_bid);
                    println!("   Best Ask: ${:.2}", best_ask);
                    println!("   Spread: ${:.2} ({:.1} bps)", spread, spread_bps);
                    println!("   Avg Spread: {:.1} bps", avg_spread);

                    // Check for low liquidity
                    if total_liquidity < min_total_liquidity {
                        println!("\n⚠️  LOW LIQUIDITY WARNING!");
                        println!(
                            "   Total: {:.2} BTC (threshold: {} BTC)",
                            total_liquidity, min_total_liquidity
                        );

                        #[cfg(feature = "telegram")]
                        if let Some(ref bot) = bot {
                            let message = format!(
                                "⚠️ {} Low Liquidity Alert\n\
                                \n\
                                Total Liquidity: {:.2} BTC\n\
                                Threshold: {} BTC\n\
                                \n\
                                Bid Volume: {:.2} BTC\n\
                                Ask Volume: {:.2} BTC\n\
                                \n\
                                💡 Low liquidity may indicate increased volatility risk.",
                                trading_pair,
                                total_liquidity,
                                min_total_liquidity,
                                total_bid_volume,
                                total_ask_volume
                            );
                            let _ = bot.send_alert(&message).await;
                        }
                    }

                    // Check for wide spread
                    if spread_bps > max_spread_bps {
                        println!("\n⚠️  WIDE SPREAD WARNING!");
                        println!(
                            "   Spread: {:.1} bps (threshold: {} bps)",
                            spread_bps, max_spread_bps
                        );

                        #[cfg(feature = "telegram")]
                        if let Some(ref bot) = bot {
                            if spread_bps > avg_spread * 2.0 {
                                let _ = bot
                                    .send_spread_alert(
                                        trading_pair,
                                        spread_bps,
                                        avg_spread,
                                        spread_bps / avg_spread,
                                    )
                                    .await;
                            }
                        }
                    }

                    // Calculate imbalance
                    #[cfg(feature = "analytics")]
                    {
                        let metrics = ob.imbalance_metrics();
                        let signal = metrics.signal(0.15);

                        println!("\n📊 Market Analytics:");
                        println!("   Bid/Ask Ratio: {:.2}", metrics.bid_ask_ratio);
                        println!("   Imbalance: {:+.2}%", metrics.imbalance_ratio * 100.0);
                        println!("   Signal: {:?}", signal);
                    }

                    println!("{}", "─".repeat(70));
                }
            }

            last_check = std::time::Instant::now();
        }
    }

    Ok(())
}
