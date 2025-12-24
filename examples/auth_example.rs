//! Authentication Example
//!
//! This example demonstrates the authentication module for Kraken WebSocket API.
//!
//! ## Features Showcased
//! - HMAC-SHA256 token generation
//! - Credentials management
//! - Private channel message structures
//!
//! ## Setup
//!
//! You'll need API credentials from Kraken:
//! 1. Log into kraken.com
//! 2. Settings → API → Create API Key
//! 3. Set permissions (view balances, view orders, etc.)
//! 4. Save the API Key and API Secret
//!
//! ## Run
//! ```bash
//! cargo run --example auth_example --features private
//! ```
//!
//! ## Note
//! This example shows the authentication infrastructure.
//! Full WebSocket integration is in development.

use kraky::{Credentials, BalanceUpdate, OrderUpdate, ExecutionUpdate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║       🔐 Kraky Authentication Example                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Create Credentials
    // ═══════════════════════════════════════════════════════════════════════

    println!("═══════════════════════════════════════════════════════════════");
    println!("  STEP 1: Creating Credentials");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Note: Replace these with your actual Kraken API credentials
    let api_key = "YOUR_API_KEY_HERE";
    let api_secret = "YOUR_API_SECRET_BASE64_HERE"; // Base64 encoded secret from Kraken

    let credentials = Credentials::new(api_key, api_secret);
    println!("✅ Credentials created");
    println!("   API Key: {}...", &credentials.api_key()[..min(20, credentials.api_key().len())]);

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Generate Authentication Token
    // ═══════════════════════════════════════════════════════════════════════

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  STEP 2: Generating Authentication Token");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Generate nonce (timestamp in nanoseconds)
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos() as u64;

    println!("📝 Nonce generated: {}", nonce);

    // Generate HMAC-SHA256 signature
    match credentials.generate_token(nonce) {
        Ok(token) => {
            println!("✅ Token generated successfully");
            println!("   Token (first 40 chars): {}...", &token[..min(40, token.len())]);
            println!("\n   This token would be sent in WebSocket subscription:");
            println!("   {{");
            println!("     \"method\": \"subscribe\",");
            println!("     \"params\": {{");
            println!("       \"channel\": \"balances\",");
            println!("       \"token\": \"{}...\"", &token[..min(20, token.len())]);
            println!("     }}");
            println!("   }}");
        }
        Err(e) => {
            println!("❌ Token generation failed: {}", e);
            println!("\n   Note: Make sure your API secret is valid base64");
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Private Channel Message Structures
    // ═══════════════════════════════════════════════════════════════════════

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  STEP 3: Private Channel Message Structures");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Example Balance Update (what you'd receive from Kraken)
    let balance_json = r#"{
        "channel": "balances",
        "type": "update",
        "data": [{
            "BTC": "1.5432",
            "USD": "50000.00",
            "ETH": "10.25"
        }]
    }"#;

    println!("💰 Balance Update Example:");
    match serde_json::from_str::<BalanceUpdate>(balance_json) {
        Ok(update) => {
            println!("   Channel: {}", update.channel);
            println!("   Assets in update: {:?}", update.assets());
            if let Some(btc) = update.get_balance("BTC") {
                println!("   BTC Balance: {}", btc);
            }
            if let Some(usd) = update.get_balance("USD") {
                println!("   USD Balance: ${}", usd);
            }
        }
        Err(e) => println!("   Parse error: {}", e),
    }

    // Example Order Update
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
            "filled_qty": "0.25",
            "status": "open",
            "timestamp": "2024-01-01T00:00:00Z"
        }]
    }"#;

    println!("\n📋 Order Update Example:");
    match serde_json::from_str::<OrderUpdate>(order_json) {
        Ok(update) => {
            println!("   Channel: {}", update.channel);
            if let Some(order) = update.data.first() {
                println!("   Order ID: {}", order.order_id);
                println!("   Pair: {}", order.symbol);
                println!("   Side: {}", order.side);
                println!("   Type: {}", order.order_type);
                println!("   Status: {}", order.status);
                println!("   Filled: {}/{}", order.filled_qty, order.order_qty);
            }
            println!("   Is Open: {}", update.is_open());
        }
        Err(e) => println!("   Parse error: {}", e),
    }

    // Example Execution Update
    let execution_json = r#"{
        "channel": "executions",
        "type": "update",
        "data": [{
            "exec_id": "E12345-ABCDE",
            "order_id": "O12345-ABCDE-FGHIJ",
            "symbol": "BTC/USD",
            "side": "buy",
            "exec_qty": "0.25",
            "exec_price": "95000.00",
            "timestamp": "2024-01-01T00:00:00Z",
            "liquidity": "taker"
        }]
    }"#;

    println!("\n💥 Execution Update Example:");
    match serde_json::from_str::<ExecutionUpdate>(execution_json) {
        Ok(update) => {
            println!("   Channel: {}", update.channel);
            if let Some(exec) = update.data.first() {
                println!("   Execution ID: {}", exec.exec_id);
                println!("   Order ID: {}", exec.order_id);
                println!("   Pair: {}", exec.symbol);
                println!("   Side: {}", exec.side);
                println!("   Quantity: {} BTC", exec.exec_qty);
                println!("   Price: ${}", exec.exec_price);
                println!("   Liquidity: {}", exec.liquidity);
            }
            if let Some(value) = update.total_value() {
                println!("   Total Value: ${:.2}", value);
            }
        }
        Err(e) => println!("   Parse error: {}", e),
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 SUMMARY                                 ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Authentication Infrastructure: ✅ Complete                   ║");
    println!("║                                                               ║");
    println!("║  ✅ HMAC-SHA256 token generation                              ║");
    println!("║  ✅ Credentials management                                    ║");
    println!("║  ✅ Balance update parsing                                    ║");
    println!("║  ✅ Order update parsing                                      ║");
    println!("║  ✅ Execution update parsing                                  ║");
    println!("║                                                               ║");
    println!("║  🚧 Coming Soon:                                              ║");
    println!("║     - WebSocket subscription integration                     ║");
    println!("║     - Real-time balance monitoring                           ║");
    println!("║     - Order placement via WebSocket                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    Ok(())
}

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}
