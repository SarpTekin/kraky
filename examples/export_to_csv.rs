//! 📊 CSV Data Export - Stream Market Data to Files
//!
//! This example demonstrates how to export live market data to CSV files
//! for analysis in Excel, Python, R, or other data analysis tools.
//!
//! ## What This Shows
//! - Real-time orderbook export to CSV
//! - Trade stream export to CSV
//! - Timestamped data for analysis
//! - Complete data pipeline: Stream → Process → Export
//!
//! ## Use Cases
//! - Backtesting strategies
//! - Market analysis in Excel/Python
//! - Building datasets for ML models
//! - Historical data collection
//! - Debugging and monitoring
//!
//! ## Setup
//! ```bash
//! cargo run --example export_to_csv --features analytics
//! ```
//!
//! ## Output Files
//! - `orderbook_BTCUSD_YYYYMMDD_HHMMSS.csv` - Orderbook snapshots
//! - `trades_BTCUSD_YYYYMMDD_HHMMSS.csv` - Individual trades
//!
//! ## CSV Format
//!
//! ### Orderbook CSV:
//! ```csv
//! timestamp,best_bid,best_ask,spread,mid_price,imbalance,bid_volume,ask_volume
//! 2024-01-15T10:30:00.123Z,42500.0,42501.5,1.5,42500.75,0.15,12.5,10.8
//! ```
//!
//! ### Trades CSV:
//! ```csv
//! timestamp,price,quantity,side,trade_id
//! 2024-01-15T10:30:00.123Z,42501.0,0.5,buy,12345678
//! ```

use chrono::Utc;
use kraky::KrakyClient;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           📊 CSV Data Export - Market Data Streaming         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Setup
    // ═══════════════════════════════════════════════════════════════════════

    let symbol = "BTC/USD";
    let duration_secs = 30; // Export for 30 seconds

    println!("⚙️  Configuration:");
    println!("   Symbol: {}", symbol);
    println!("   Duration: {} seconds", duration_secs);
    println!("   Export: Orderbook + Trades\n");

    // Create timestamped filenames
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let orderbook_file = format!("orderbook_BTCUSD_{}.csv", timestamp);
    let trades_file = format!("trades_BTCUSD_{}.csv", timestamp);

    println!("📁 Output Files:");
    println!("   Orderbook: {}", orderbook_file);
    println!("   Trades: {}\n", trades_file);

    // Create CSV files with headers
    let mut ob_csv = File::create(&orderbook_file)?;
    let mut trades_csv = File::create(&trades_file)?;

    // Write CSV headers
    writeln!(
        ob_csv,
        "timestamp,best_bid,best_ask,spread,mid_price,imbalance,bid_volume,ask_volume"
    )?;
    writeln!(trades_csv, "timestamp,price,quantity,side,trade_id")?;

    println!("✅ CSV files created with headers\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Connect to Kraken
    // ═══════════════════════════════════════════════════════════════════════

    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Subscribe to Market Data
    // ═══════════════════════════════════════════════════════════════════════

    println!("📊 Subscribing to market data...");
    let mut orderbook_sub = client.subscribe_orderbook(symbol, 10).await?;
    let mut trades_sub = client.subscribe_trades(symbol).await?;
    println!("✅ Subscribed to orderbook and trades\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 4: Stream and Export Data
    // ═══════════════════════════════════════════════════════════════════════

    println!("🚀 Streaming data to CSV files...");
    println!("   Press Ctrl+C to stop\n");
    println!("{}", "═".repeat(70));

    let start_time = tokio::time::Instant::now();
    let duration = tokio::time::Duration::from_secs(duration_secs);

    let mut orderbook_count = 0;
    let mut trades_count = 0;

    loop {
        // Check if duration exceeded
        if start_time.elapsed() >= duration {
            println!("\n{}", "═".repeat(70));
            println!("⏰ Duration limit reached");
            break;
        }

        tokio::select! {
            // Handle orderbook updates
            Some(_) = orderbook_sub.next() => {
                if let Some(ob) = client.get_orderbook(symbol) {
                    let timestamp = Utc::now().to_rfc3339();
                    let best_bid = ob.best_bid().unwrap_or(0.0);
                    let best_ask = ob.best_ask().unwrap_or(0.0);
                    let spread = ob.spread().unwrap_or(0.0);
                    let mid_price = ob.mid_price().unwrap_or(0.0);

                    #[cfg(feature = "analytics")]
                    let imbalance = ob.imbalance();
                    #[cfg(not(feature = "analytics"))]
                    let imbalance = 0.0;

                    #[cfg(feature = "analytics")]
                    let metrics = ob.imbalance_metrics();
                    #[cfg(feature = "analytics")]
                    let bid_volume = metrics.bid_volume;
                    #[cfg(feature = "analytics")]
                    let ask_volume = metrics.ask_volume;
                    #[cfg(not(feature = "analytics"))]
                    let bid_volume = 0.0;
                    #[cfg(not(feature = "analytics"))]
                    let ask_volume = 0.0;

                    // Write to CSV
                    writeln!(
                        ob_csv,
                        "{},{},{},{},{},{:.4},{:.4},{:.4}",
                        timestamp, best_bid, best_ask, spread, mid_price,
                        imbalance, bid_volume, ask_volume
                    )?;

                    orderbook_count += 1;

                    if orderbook_count % 10 == 0 {
                        print!("\r📖 Orderbook: {} records | 💱 Trades: {} records",
                            orderbook_count, trades_count);
                        std::io::stdout().flush()?;
                    }
                }
            }

            // Handle trade updates
            Some(trade) = trades_sub.next() => {
                let timestamp = Utc::now().to_rfc3339();

                // Write to CSV
                writeln!(
                    trades_csv,
                    "{},{},{},{},{}",
                    timestamp,
                    trade.price,
                    trade.qty,
                    match trade.side {
                        kraky::TradeSide::Buy => "buy",
                        kraky::TradeSide::Sell => "sell",
                    },
                    trade.trade_id
                )?;

                trades_count += 1;

                print!("\r📖 Orderbook: {} records | 💱 Trades: {} records",
                    orderbook_count, trades_count);
                std::io::stdout().flush()?;
            }

            else => break,
        }
    }

    println!("\n{}", "═".repeat(70));

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 5: Finalize and Summary
    // ═══════════════════════════════════════════════════════════════════════

    // Ensure all data is written
    ob_csv.flush()?;
    trades_csv.flush()?;

    println!("\n✅ Export completed!\n");

    println!("📊 Summary:");
    println!("   Orderbook records: {}", orderbook_count);
    println!("   Trade records: {}", trades_count);
    println!("   Duration: {:.1}s", start_time.elapsed().as_secs_f64());
    println!();

    println!("📁 Files Created:");
    println!("   {}", orderbook_file);
    println!("   {}", trades_file);
    println!();

    println!("💡 Next Steps:");
    println!("   - Open CSV files in Excel/Google Sheets");
    println!(
        "   - Import into Python with: pd.read_csv('{}')\"",
        orderbook_file
    );
    println!("   - Analyze spread, imbalance, and trade patterns");
    println!("   - Use for backtesting or ML model training");
    println!();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                  📊 Export Complete! 🎉                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    Ok(())
}
