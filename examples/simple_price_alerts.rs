//! 🔔 Simple Price Alerts
//!
//! The easiest way to get started with Kraky SDK.
//! Get Telegram notifications when BTC crosses price thresholds.
//!
//! ## Features
//! - Simple configuration
//! - Price threshold alerts
//! - Telegram notifications
//! - Perfect for beginners
//!
//! ## Run
//! ```bash
//! export TELEGRAM_BOT_TOKEN="your_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! cargo run --example simple_price_alerts --features telegram-alerts
//! ```

use kraky::KrakyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           🔔 Simple Price Alerts - Demo                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Configuration
    // ═══════════════════════════════════════════════════════════════════════

    // What price levels should trigger alerts?
    let price_high = 100_000.0;  // Alert if BTC goes above $100k
    let price_low = 90_000.0;    // Alert if BTC goes below $90k

    println!("⚙️  Alert Configuration:");
    println!("   High Alert: ${:.2}", price_high);
    println!("   Low Alert:  ${:.2}\n", price_low);

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Setup Telegram
    // ═══════════════════════════════════════════════════════════════════════

    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");
    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .expect("Please set TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");

    #[cfg(feature = "telegram")]
    let bot = kraky::TelegramNotifier::new(&bot_token, chat_id);

    println!("📱 Telegram bot ready");
    println!("   Chat ID: {}\n", chat_id);

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Connect to Kraken
    // ═══════════════════════════════════════════════════════════════════════

    println!("📡 Connecting to Kraken...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // Send startup notification
    #[cfg(feature = "telegram")]
    bot.send_connection_status(
        true,
        &format!("🔔 Price Alert Bot started!\n\
                 High: ${:.2}\n\
                 Low: ${:.2}",
                 price_high, price_low)
    ).await?;

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 4: Subscribe to Price Updates
    // ═══════════════════════════════════════════════════════════════════════

    println!("📊 Subscribing to BTC/USD ticker...");
    let mut ticker = client.subscribe_ticker("BTC/USD").await?;
    println!("✅ Subscribed\n");

    println!("🚀 Price Alert Bot is now running!");
    println!("   Waiting for price to cross thresholds...");
    println!("   Press Ctrl+C to stop\n");
    println!("{}", "═".repeat(60));

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 5: Monitor Prices and Send Alerts
    // ═══════════════════════════════════════════════════════════════════════

    let mut alert_sent_high = false;
    let mut alert_sent_low = false;
    let mut update_count = 0;

    while let Some(tick) = ticker.next().await {
        update_count += 1;
        let current_price = tick.last;

        // Display status every 20 updates
        if update_count % 20 == 0 {
            println!("\n📈 Current Price: ${:.2} (24h: {:+.2}%)",
                current_price, tick.change_pct);
        }

        // Check HIGH threshold
        if current_price >= price_high && !alert_sent_high {
            println!("\n🚨 HIGH PRICE ALERT!");
            println!("   Current: ${:.2}", current_price);
            println!("   Threshold: ${:.2}", price_high);

            #[cfg(feature = "telegram")]
            {
                bot.send_threshold_alert(
                    "BTC/USD",
                    current_price,
                    price_high,
                    true  // above threshold
                ).await?;

                println!("   ✅ Telegram alert sent!");
            }

            alert_sent_high = true;
            alert_sent_low = false; // Reset low alert
        }

        // Check LOW threshold
        if current_price <= price_low && !alert_sent_low {
            println!("\n🚨 LOW PRICE ALERT!");
            println!("   Current: ${:.2}", current_price);
            println!("   Threshold: ${:.2}", price_low);

            #[cfg(feature = "telegram")]
            {
                bot.send_threshold_alert(
                    "BTC/USD",
                    current_price,
                    price_low,
                    false  // below threshold
                ).await?;

                println!("   ✅ Telegram alert sent!");
            }

            alert_sent_low = true;
            alert_sent_high = false; // Reset high alert
        }
    }

    Ok(())
}
