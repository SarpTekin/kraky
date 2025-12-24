//! 🤖 Telegram Trading Demo - Showcase All Features
//!
//! This example demonstrates the complete trading notification system
//! WITHOUT requiring Kraken API credentials. Perfect for:
//! - Hackathon demonstrations
//! - Testing Telegram integration
//! - Showcasing SDK capabilities
//!
//! ## Setup
//!
//! ### 1. Setup Telegram Bot (if not done already)
//! - Message @BotFather on Telegram
//! - Create a new bot with /newbot
//! - Get your chat ID from @userinfobot
//!
//! ### 2. Set Environment Variables
//! ```bash
//! export TELEGRAM_BOT_TOKEN="your_bot_token"
//! export TELEGRAM_CHAT_ID="your_chat_id"
//! ```
//!
//! ### 3. Run the Demo
//! ```bash
//! cargo run --example telegram_trading_demo --features telegram,trading
//! ```
//!
//! ## What This Demonstrates
//! - All 6 types of trading notifications
//! - WebSocket architecture (simulated)
//! - Order lifecycle management
//! - Real-time Telegram alerts
//! - Complete trading workflow

use kraky::{
    OrderParams, OrderSide, OrderStatus,
    TelegramNotifier, AmendOrderParams,
    OrderResponse, AmendOrderResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║        🤖 Telegram Trading Demo - Feature Showcase          ║");
    println!("║                  (No Kraken Account Needed)                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Load Telegram Configuration
    // ═══════════════════════════════════════════════════════════════════════

    println!("⚙️  Loading Telegram configuration...\n");

    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");
    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .expect("Please set TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");

    let bot = TelegramNotifier::new(&bot_token, chat_id);

    println!("✅ Telegram bot configured!");
    println!("   Bot Token: {}...", &bot_token[..20]);
    println!("   Chat ID: {}\n", chat_id);

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Send Connection Notification
    // ═══════════════════════════════════════════════════════════════════════

    println!("📡 Simulating WebSocket connection...");
    bot.send_connection_status(
        true,
        "🤖 Trading Bot Demo Started!\n\
        Mode: ✅ Demonstration Mode\n\
        All features showcased via Telegram notifications"
    ).await?;
    println!("✅ Connection notification sent!\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Demonstrate All Trading Notifications
    // ═══════════════════════════════════════════════════════════════════════

    println!("🚀 Demonstrating trading features...\n");
    println!("{}", "═".repeat(70));

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 1: Market Buy Order Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("\n📌 DEMO 1: Market Buy Order Notification");
    println!("{}", "─".repeat(70));

    let market_buy = OrderParams::market_buy("BTC/USD", 0.001)
        .with_validate(true);

    let market_response = OrderResponse {
        order_id: "OBUY-12345-ABCDEF".to_string(),
        cl_ord_id: None,
        order_status: OrderStatus::Pending,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    println!("   Sending market buy order notification...");
    println!("   Symbol: BTC/USD");
    println!("   Quantity: 0.001 BTC");
    println!("   Type: Market");

    bot.send_order_placed(&market_response, &market_buy).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 2: Limit Sell Order Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 2: Limit Sell Order Notification");
    println!("{}", "─".repeat(70));

    let limit_sell = OrderParams::limit_sell("BTC/USD", 0.001, 105000.0)
        .with_validate(true)
        .with_client_id("demo-limit-sell-001");

    let limit_response = OrderResponse {
        order_id: "OSELL-67890-GHIJK".to_string(),
        cl_ord_id: Some("demo-limit-sell-001".to_string()),
        order_status: OrderStatus::Pending,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    println!("   Sending limit sell order notification...");
    println!("   Symbol: BTC/USD");
    println!("   Quantity: 0.001 BTC");
    println!("   Type: Limit");
    println!("   Price: $105,000.00");

    bot.send_order_placed(&limit_response, &limit_sell).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 3: Order Amendment Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 3: Order Amendment Notification");
    println!("{}", "─".repeat(70));

    let amend_params = AmendOrderParams {
        order_id: limit_response.order_id.clone(),
        order_qty: None,
        limit_price: Some(106000.0),
        trigger_price: None,
    };

    let amend_response = AmendOrderResponse {
        order_id: limit_response.order_id.clone(),
        success: true,
        error: None,
    };

    println!("   Sending amendment notification...");
    println!("   Order ID: {}", limit_response.order_id);
    println!("   New Price: $106,000.00");

    bot.send_order_amended(&amend_response, &amend_params).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 4: Order Cancellation Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 4: Order Cancellation Notification");
    println!("{}", "─".repeat(70));

    println!("   Sending cancellation notification...");
    println!("   Order ID: {}", limit_response.order_id);
    println!("   Reason: Demo completed");

    bot.send_order_cancelled(
        "BTC/USD",
        &limit_response.order_id,
        Some("Demo completed - showcasing cancellation flow")
    ).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 5: Order Fill Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 5: Order Fill Notification");
    println!("{}", "─".repeat(70));

    println!("   Sending order fill notification...");
    println!("   Symbol: BTC/USD");
    println!("   Side: Buy");
    println!("   Quantity: 0.001 BTC");
    println!("   Price: $100,500.00");

    bot.send_order_filled(
        "BTC/USD",
        &OrderSide::Buy,
        0.001,
        100500.0,
        "OFILL-11111-ZZZZZ"
    ).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 6: Order Failure Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 6: Order Failure Notification");
    println!("{}", "─".repeat(70));

    let failed_order = OrderParams::market_buy("ETH/USD", 10.0);

    println!("   Sending order failure notification...");
    println!("   Symbol: ETH/USD");
    println!("   Error: Insufficient balance");

    bot.send_order_failed(
        &failed_order,
        "Insufficient balance: Required 25,000 USD, Available 10,000 USD"
    ).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ───────────────────────────────────────────────────────────────────────
    // Demo 7: Trading Summary Notification
    // ───────────────────────────────────────────────────────────────────────

    println!("📌 DEMO 7: Daily Trading Summary");
    println!("{}", "─".repeat(70));

    println!("   Sending daily trading summary...");
    println!("   Total Trades: 12");
    println!("   Total Volume: $3,456.78");
    println!("   Profit/Loss: +$234.56");
    println!("   Win Rate: 75%");

    bot.send_trading_summary(
        12,         // 12 trades today
        3456.78,    // $3,456.78 volume
        234.56,     // +$234.56 profit
        75.0        // 75% win rate
    ).await?;
    println!("   ✅ Notification sent to Telegram!\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // ═══════════════════════════════════════════════════════════════════════
    // COMPLETE
    // ═══════════════════════════════════════════════════════════════════════

    println!("{}", "═".repeat(70));
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    🎉 DEMO COMPLETE!                          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Features Demonstrated:                                       ║");
    println!("║    ✅ Market order placement notification                     ║");
    println!("║    ✅ Limit order placement notification                      ║");
    println!("║    ✅ Order amendment notification                            ║");
    println!("║    ✅ Order cancellation notification                         ║");
    println!("║    ✅ Order fill notification                                 ║");
    println!("║    ✅ Order failure notification                              ║");
    println!("║    ✅ Daily trading summary                                   ║");
    println!("║                                                                ║");
    println!("║  Check your Telegram for 8 notifications! 📱                 ║");
    println!("║                                                                ║");
    println!("║  This demonstrates the complete WebSocket trading workflow    ║");
    println!("║  with real-time Telegram alerts - perfect for hackathons! 🏆 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Send completion notification
    bot.send_connection_status(
        false,
        "🤖 Trading Bot Demo Completed!\n\
        \n\
        All 7 notification types showcased:\n\
        ✅ Order Placed\n\
        ✅ Order Amended\n\
        ✅ Order Cancelled\n\
        ✅ Order Filled\n\
        ✅ Order Failed\n\
        ✅ Trading Summary\n\
        ✅ Connection Status\n\
        \n\
        Ready for your hackathon presentation! 🏆"
    ).await?;

    println!("👋 Demo completed successfully!\n");
    println!("💡 TIP: The real SDK connects to Kraken's WebSocket API v2");
    println!("         and performs all these operations for real!\n");

    Ok(())
}
