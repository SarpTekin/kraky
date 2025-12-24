//! 🐋 Whale Watcher - Large Order Detection Bot
//!
//! Monitors the orderbook for large orders ("whales") and sends Telegram alerts.
//! Perfect for traders who want to know when big players enter the market.
//!
//! ## Features
//! - Real-time whale detection
//! - Configurable volume thresholds
//! - Telegram notifications
//! - Multiple trading pairs support
//!
//! ## Run
//! ```bash
//! export TELEGRAM_BOT_TOKEN="your_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! cargo run --example whale_watcher --features telegram-alerts
//! ```

use kraky::KrakyClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              🐋 Whale Watcher - Demo                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Get Telegram credentials
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");
    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .expect("Please set TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");

    #[cfg(feature = "telegram")]
    let bot = kraky::TelegramNotifier::new(&bot_token, chat_id);

    // Configuration
    let trading_pair = "BTC/USD";
    let whale_threshold_btc = 1.0; // Orders >= 10 BTC are "whales"
    let check_interval = Duration::from_secs(5);

    println!("⚙️  Configuration:");
    println!("   Trading Pair: {}", trading_pair);
    println!("   Whale Threshold: {} BTC", whale_threshold_btc);
    println!("   Check Interval: {:?}\n", check_interval);

    // Connect to Kraken
    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // Send startup notification
    #[cfg(feature = "telegram")]
    bot.send_connection_status(
        true,
        &format!(
            "🐋 Whale Watcher started!\nMonitoring {} for orders >= {} BTC",
            trading_pair, whale_threshold_btc
        ),
    )
    .await?;

    // Subscribe to orderbook
    println!(
        "📊 Subscribing to {} orderbook (depth: 25)...",
        trading_pair
    );
    let mut orderbook_sub = client.subscribe_orderbook(trading_pair, 25).await?;
    println!("✅ Subscribed\n");

    println!("🚀 Whale Watcher is now active!");
    println!("   Monitoring for orders >= {} BTC", whale_threshold_btc);
    println!("   Press Ctrl+C to stop\n");

    // Track state
    let mut update_count = 0;
    let mut whale_count = 0;
    let mut last_check = std::time::Instant::now();

    // Main loop
    while let Some(_update) = orderbook_sub.next().await {
        update_count += 1;

        // Only check periodically to avoid spam
        if last_check.elapsed() >= check_interval {
            if let Some(ob) = client.get_orderbook(trading_pair) {
                // Check top 10 bids for whales
                for (i, (price, volume)) in ob.bids.iter().take(10).enumerate() {
                    if *volume >= whale_threshold_btc {
                        whale_count += 1;
                        let price_f64 = price.0;

                        println!("🐋 WHALE DETECTED!");
                        println!("   Side: BID (Buy)");
                        println!("   Position: #{}", i + 1);
                        println!("   Volume: {:.4} BTC", volume);
                        println!("   Price: ${:.2}", price_f64);
                        println!("   Total Value: ${:.2}\n", volume * price_f64);

                        #[cfg(feature = "telegram")]
                        {
                            bot.send_whale_alert(trading_pair, "bid", price_f64, *volume)
                                .await?;
                        }

                        break; // Only alert once per check
                    }
                }

                // Check top 10 asks for whales
                for (i, (price, volume)) in ob.asks.iter().take(10).enumerate() {
                    if *volume >= whale_threshold_btc {
                        whale_count += 1;
                        let price_f64 = price.0;

                        println!("🐋 WHALE DETECTED!");
                        println!("   Side: ASK (Sell)");
                        println!("   Position: #{}", i + 1);
                        println!("   Volume: {:.4} BTC", volume);
                        println!("   Price: ${:.2}", price_f64);
                        println!("   Total Value: ${:.2}\n", volume * price_f64);

                        #[cfg(feature = "telegram")]
                        {
                            bot.send_whale_alert(trading_pair, "ask", price_f64, *volume)
                                .await?;
                        }

                        break; // Only alert once per check
                    }
                }

                // Periodic status update
                if update_count % 100 == 0 {
                    println!(
                        "📊 Status: {} updates | {} whales detected",
                        update_count, whale_count
                    );
                }
            }

            last_check = std::time::Instant::now();
        }
    }

    Ok(())
}
