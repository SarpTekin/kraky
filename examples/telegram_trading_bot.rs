//! 🤖 Telegram Trading Bot - Complete Trading Integration Demo
//!
//! This example demonstrates the full trading capabilities of Kraky SDK:
//! - Place market and limit orders via WebSocket
//! - Cancel and amend orders
//! - Real-time Telegram notifications for all trading events
//! - Order validation (dry-run mode)
//!
//! ## Features Demonstrated
//! 1. Order placement (market & limit orders)
//! 2. Order cancellation
//! 3. Order amendment (modification)
//! 4. Telegram notifications for all events
//! 5. Error handling and validation
//!
//! ## Setup
//!
//! ### 1. Get API Credentials
//! - Sign up at https://www.kraken.com
//! - Go to Settings > API
//! - Create new API key with trading permissions
//! - Save your API key and secret
//!
//! ### 2. Setup Telegram Bot
//! - Message @BotFather on Telegram
//! - Create a new bot with /newbot
//! - Get your chat ID from @userinfobot
//!
//! ### 3. Set Environment Variables
//! ```bash
//! export KRAKEN_API_KEY="your_api_key"
//! export KRAKEN_API_SECRET="your_api_secret"
//! export TELEGRAM_BOT_TOKEN="your_bot_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! ```
//!
//! ### 4. Run the Example
//! ```bash
//! # With validation mode (safe - no real orders placed)
//! cargo run --example telegram_trading_bot --features telegram,trading
//! ```
//!
//! ## Safety Features
//! - Uses VALIDATION MODE by default (dry-run, no real trades)
//! - All orders are checked but not executed
//! - Perfect for testing and demos
//! - To enable real trading, set ENABLE_REAL_TRADING=true

use kraky::{
    AmendOrderParams, Credentials, KrakyClient, OrderParams, OrderSide, OrderType, TelegramNotifier,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           🤖 Telegram Trading Bot - Demo                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Load Configuration
    // ═══════════════════════════════════════════════════════════════════════

    println!("⚙️  Loading configuration...\n");

    // Kraken API credentials
    let api_key =
        std::env::var("KRAKEN_API_KEY").expect("Please set KRAKEN_API_KEY environment variable");
    let api_secret = std::env::var("KRAKEN_API_SECRET")
        .expect("Please set KRAKEN_API_SECRET environment variable");

    let credentials = Credentials::new(api_key, api_secret);

    // Telegram bot
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");
    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .expect("Please set TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");

    let bot = TelegramNotifier::new(&bot_token, chat_id);

    // Safety: Check if real trading is enabled (default: validation mode only)
    let enable_real_trading = std::env::var("ENABLE_REAL_TRADING")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if enable_real_trading {
        println!("⚠️  REAL TRADING MODE ENABLED");
        println!("   Orders will be executed on the exchange");
    } else {
        println!("✅ VALIDATION MODE (Safe)");
        println!("   Orders will be validated but NOT executed");
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Connect to Kraken
    // ═══════════════════════════════════════════════════════════════════════

    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // Send startup notification
    bot.send_connection_status(
        true,
        &format!(
            "🤖 Trading Bot started!\n\
            Mode: {}\n\
            Ready to execute orders via WebSocket",
            if enable_real_trading {
                "⚠️ LIVE TRADING"
            } else {
                "✅ Validation Only"
            }
        ),
    )
    .await?;

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Demonstrate Trading Operations
    // ═══════════════════════════════════════════════════════════════════════

    println!("🚀 Starting trading demonstrations...\n");
    println!("{}", "═".repeat(70));

    // ───────────────────────────────────────────────────────────────────────
    // Demo 1: Market Buy Order
    // ───────────────────────────────────────────────────────────────────────

    println!("\n📌 DEMO 1: Market Buy Order");
    println!("{}", "─".repeat(70));

    let market_buy = OrderParams::market_buy("BTC/USD", 0.001).with_validate(!enable_real_trading); // Validate only unless real trading enabled

    println!("   Placing market buy order...");
    println!("   Symbol: BTC/USD");
    println!("   Quantity: 0.001 BTC");
    println!("   Type: Market");

    match client.place_order(&credentials, market_buy.clone()).await {
        Ok(response) => {
            println!("   ✅ Order placed!");
            println!("   Order ID: {}", response.order_id);
            println!("   Status: {:?}", response.order_status);

            // Send Telegram notification
            bot.send_order_placed(&response, &market_buy).await?;
        }
        Err(e) => {
            println!("   ❌ Order failed: {}", e);
            bot.send_order_failed(&market_buy, &e.to_string()).await?;
        }
    }

    // Wait a bit between operations
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 2: Limit Sell Order
    // ───────────────────────────────────────────────────────────────────────

    println!("\n📌 DEMO 2: Limit Sell Order");
    println!("{}", "─".repeat(70));

    let limit_sell = OrderParams::limit_sell("BTC/USD", 0.001, 105000.0)
        .with_validate(!enable_real_trading)
        .with_client_id("demo-limit-sell-001");

    println!("   Placing limit sell order...");
    println!("   Symbol: BTC/USD");
    println!("   Quantity: 0.001 BTC");
    println!("   Type: Limit");
    println!("   Price: $105,000.00");

    match client.place_order(&credentials, limit_sell.clone()).await {
        Ok(response) => {
            println!("   ✅ Order placed!");
            println!("   Order ID: {}", response.order_id);
            println!("   Client ID: {:?}", response.cl_ord_id);

            bot.send_order_placed(&response, &limit_sell).await?;

            // ───────────────────────────────────────────────────────────────
            // Demo 3: Amend Order
            // ───────────────────────────────────────────────────────────────

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            println!("\n📌 DEMO 3: Amend Order (Change Price)");
            println!("{}", "─".repeat(70));

            let amend = AmendOrderParams {
                order_id: response.order_id.clone(),
                order_qty: None,
                limit_price: Some(106000.0), // Increase price
                trigger_price: None,
            };

            println!("   Amending order {}...", response.order_id);
            println!("   New Limit Price: $106,000.00");

            match client.amend_order(&credentials, amend.clone()).await {
                Ok(amend_response) => {
                    println!("   ✅ Order amended!");
                    println!("   Success: {}", amend_response.success);

                    bot.send_order_amended(&amend_response, &amend).await?;
                }
                Err(e) => {
                    println!("   ❌ Amendment failed: {}", e);
                }
            }

            // ───────────────────────────────────────────────────────────────
            // Demo 4: Cancel Order
            // ───────────────────────────────────────────────────────────────

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            println!("\n📌 DEMO 4: Cancel Order");
            println!("{}", "─".repeat(70));

            println!("   Cancelling order {}...", response.order_id);

            match client.cancel_order(&credentials, &response.order_id).await {
                Ok(cancel_response) => {
                    println!("   ✅ Order cancelled!");
                    println!("   Success: {}", cancel_response.success);

                    bot.send_order_cancelled("BTC/USD", &response.order_id, Some("Demo completed"))
                        .await?;
                }
                Err(e) => {
                    println!("   ❌ Cancellation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Order failed: {}", e);
            bot.send_order_failed(&limit_sell, &e.to_string()).await?;
        }
    }

    // ───────────────────────────────────────────────────────────────────────
    // Demo 5: Simulate Order Fill Notification
    // ───────────────────────────────────────────────────────────────────────

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n📌 DEMO 5: Order Fill Notification (Simulated)");
    println!("{}", "─".repeat(70));

    println!("   Simulating order fill...");
    bot.send_order_filled("BTC/USD", &OrderSide::Buy, 0.001, 100500.0, "demo-fill-001")
        .await?;
    println!("   ✅ Fill notification sent to Telegram");

    // ───────────────────────────────────────────────────────────────────────
    // Demo 6: Trading Summary
    // ───────────────────────────────────────────────────────────────────────

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("\n📌 DEMO 6: Daily Trading Summary");
    println!("{}", "─".repeat(70));

    println!("   Sending daily summary...");
    bot.send_trading_summary(
        5,       // 5 trades today
        1250.50, // $1,250.50 volume
        45.75,   // +$45.75 profit
        80.0,    // 80% win rate
    )
    .await?;
    println!("   ✅ Summary sent to Telegram");

    // ═══════════════════════════════════════════════════════════════════════
    // COMPLETE
    // ═══════════════════════════════════════════════════════════════════════

    println!("\n{}", "═".repeat(70));
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    🎉 DEMO COMPLETE!                          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Features Demonstrated:                                       ║");
    println!("║    ✅ Market order placement                                  ║");
    println!("║    ✅ Limit order placement                                   ║");
    println!("║    ✅ Order amendment (price change)                          ║");
    println!("║    ✅ Order cancellation                                      ║");
    println!("║    ✅ Telegram notifications (6 types)                        ║");
    println!("║    ✅ Order validation (dry-run mode)                         ║");
    println!("║                                                                ║");
    println!("║  All operations performed via WebSocket API ⚡                ║");
    println!("║  Core SDK remains lightweight (~3KB for trading) 🪶           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Send completion notification
    bot.send_connection_status(false, "🤖 Trading Bot demo completed successfully!")
        .await?;

    client.disconnect();
    println!("👋 Disconnected from Kraken\n");

    Ok(())
}
