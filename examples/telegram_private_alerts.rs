//! Telegram Private Alerts Example
//!
//! This example demonstrates Telegram notifications for private WebSocket events:
//! - Balance updates
//! - Order updates (opened, filled, cancelled)
//! - Execution alerts (trade fills)
//! - Portfolio summaries
//!
//! ## Features Showcased
//! - Real-time balance change notifications
//! - Order status notifications
//! - Trade execution alerts
//! - Portfolio summary reports
//!
//! ## Setup
//!
//! 1. Create a Telegram bot with @BotFather
//! 2. Get your chat ID (use @userinfobot)
//! 3. Set environment variables:
//!    ```bash
//!    export TELEGRAM_BOT_TOKEN="your_bot_token"
//!    export TELEGRAM_CHAT_ID="your_chat_id"
//!    ```
//! 4. Run the example:
//!    ```bash
//!    cargo run --example telegram_private_alerts --features telegram,private
//!    ```

use kraky::{BalanceUpdate, ExecutionUpdate, OrderUpdate, TelegramNotifier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║       🔐 Telegram Private Alerts - Demo                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Get Telegram credentials
    let bot_token =
        std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| "YOUR_BOT_TOKEN".to_string());
    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .unwrap_or_else(|_| "123456789".to_string())
        .parse()
        .unwrap_or(123456789);

    let bot = TelegramNotifier::new(&bot_token, chat_id);

    println!("📱 Telegram bot initialized");
    println!("   Chat ID: {}\n", chat_id);

    // ═══════════════════════════════════════════════════════════════════════
    // DEMO 1: Balance Update Notification
    // ═══════════════════════════════════════════════════════════════════════

    println!("═══════════════════════════════════════════════════════════════");
    println!("  DEMO 1: Balance Update Notification");
    println!("═══════════════════════════════════════════════════════════════\n");

    let balance_json = r#"{
        "channel": "balances",
        "type": "update",
        "data": [{
            "BTC": "1.5432",
            "ETH": "10.25",
            "USD": "50000.00"
        }]
    }"#;

    let balance_update: BalanceUpdate = serde_json::from_str(balance_json)?;
    println!("✅ Simulated balance update");
    println!("   Sending to Telegram...");

    if bot_token != "YOUR_BOT_TOKEN" {
        bot.send_balance_update(&balance_update).await?;
        println!("✅ Balance notification sent!\n");
    } else {
        println!("⚠️  Skipped (set TELEGRAM_BOT_TOKEN to send)\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DEMO 2: Order Update Notification
    // ═══════════════════════════════════════════════════════════════════════

    println!("═══════════════════════════════════════════════════════════════");
    println!("  DEMO 2: Order Update Notification");
    println!("═══════════════════════════════════════════════════════════════\n");

    let order_json = r#"{
        "channel": "orders",
        "type": "update",
        "data": [{
            "order_id": "O12345-ABCDE-FGHIJ",
            "symbol": "BTC/USD",
            "side": "buy",
            "order_type": "limit",
            "limit_price": "95000.00",
            "order_qty": "0.5",
            "filled_qty": "0.0",
            "status": "open",
            "timestamp": "2024-12-23T00:00:00Z"
        }]
    }"#;

    let order_update: OrderUpdate = serde_json::from_str(order_json)?;
    println!("✅ Simulated order opened");
    println!("   Order: Buy 0.5 BTC @ $95,000");
    println!("   Sending to Telegram...");

    if bot_token != "YOUR_BOT_TOKEN" {
        bot.send_order_update(&order_update).await?;
        println!("✅ Order notification sent!\n");
    } else {
        println!("⚠️  Skipped (set TELEGRAM_BOT_TOKEN to send)\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DEMO 3: Execution Alert (Trade Fill)
    // ═══════════════════════════════════════════════════════════════════════

    println!("═══════════════════════════════════════════════════════════════");
    println!("  DEMO 3: Execution Alert (Trade Fill)");
    println!("═══════════════════════════════════════════════════════════════\n");

    let execution_json = r#"{
        "channel": "executions",
        "type": "update",
        "data": [{
            "exec_id": "E12345-ABCDE",
            "order_id": "O12345-ABCDE-FGHIJ",
            "symbol": "BTC/USD",
            "side": "buy",
            "exec_qty": "0.5",
            "exec_price": "95000.00",
            "timestamp": "2024-12-23T00:05:00Z",
            "liquidity": "taker"
        }]
    }"#;

    let execution: ExecutionUpdate = serde_json::from_str(execution_json)?;
    println!("✅ Simulated trade execution");
    println!("   Bought: 0.5 BTC @ $95,000");
    println!("   Total: $47,500");
    println!("   Sending to Telegram...");

    if bot_token != "YOUR_BOT_TOKEN" {
        bot.send_execution_alert(&execution).await?;
        println!("✅ Execution alert sent!\n");
    } else {
        println!("⚠️  Skipped (set TELEGRAM_BOT_TOKEN to send)\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DEMO 4: Portfolio Summary
    // ═══════════════════════════════════════════════════════════════════════

    println!("═════════════════════════════════════════════════════════════");
    println!("  DEMO 4: Portfolio Summary");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✅ Generating portfolio summary");
    println!("   Sending to Telegram...");

    if bot_token != "YOUR_BOT_TOKEN" {
        bot.send_portfolio_summary(&balance_update).await?;
        println!("✅ Portfolio summary sent!\n");
    } else {
        println!("⚠️  Skipped (set TELEGRAM_BOT_TOKEN to send)\n");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 SUMMARY                                 ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Private Telegram Notifications: ✅ Complete                  ║");
    println!("║                                                               ║");
    println!("║  Notification Types:                                          ║");
    println!("║  ✅ Balance updates (real-time)                               ║");
    println!("║  ✅ Order status changes (open/filled/cancelled)              ║");
    println!("║  ✅ Trade executions (fills)                                  ║");
    println!("║  ✅ Portfolio summaries                                       ║");
    println!("║                                                               ║");
    println!("║  🎯 Use Cases:                                                ║");
    println!("║     - Get notified when orders fill                          ║");
    println!("║     - Track balance changes in real-time                     ║");
    println!("║     - Monitor portfolio without opening exchange             ║");
    println!("║     - Alert on unexpected trades                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    if bot_token == "YOUR_BOT_TOKEN" {
        println!("💡 To actually send notifications:");
        println!("   export TELEGRAM_BOT_TOKEN=\"your_token\"");
        println!("   export TELEGRAM_CHAT_ID=\"your_chat_id\"");
        println!("   cargo run --example telegram_private_alerts --features telegram,private\n");
    }

    Ok(())
}
