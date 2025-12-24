//! 📊 Multi-Pair CSV Export - Stream Multiple Assets to Files
//!
//! This example demonstrates exporting live market data from multiple
//! trading pairs simultaneously to separate CSV files.
//!
//! ## What This Shows
//! - Multi-pair data streaming
//! - Concurrent CSV export
//! - Efficient resource usage
//! - Scalable architecture
//!
//! ## Use Cases
//! - Multi-asset portfolio analysis
//! - Cross-pair correlation studies
//! - Market-wide dataset collection
//! - Comparative analysis across assets
//!
//! ## Setup
//! ```bash
//! cargo run --example export_multi_csv --features analytics
//! ```
//!
//! ## Output Files (per pair)
//! - `orderbook_<PAIR>_<TIMESTAMP>.csv` - Orderbook snapshots
//!
//! ## CSV Format
//!
//! ### Orderbook CSV:
//! ```csv
//! timestamp,pair,best_bid,best_ask,spread,mid_price,imbalance,bid_volume,ask_volume
//! 2024-01-15T10:30:00.123Z,BTC/USD,42500.0,42501.5,1.5,42500.75,0.15,12.5,10.8
//! ```

use kraky::KrakyClient;
use chrono::Utc;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

struct PairExporter {
    orderbook_file: File,
    orderbook_count: usize,
}

impl PairExporter {
    fn new(pair: &str, timestamp: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pair_filename = pair.replace('/', "");
        let orderbook_filename = format!("orderbook_{}_{}.csv", pair_filename, timestamp);
        let mut orderbook_file = File::create(&orderbook_filename)?;

        // Write headers
        writeln!(
            orderbook_file,
            "timestamp,pair,best_bid,best_ask,spread,mid_price,imbalance,bid_volume,ask_volume"
        )?;

        Ok(Self {
            orderbook_file,
            orderbook_count: 0,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║      📊 Multi-Pair CSV Export - Market Data Streaming       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Configuration
    // ═══════════════════════════════════════════════════════════════════════

    let pairs = vec!["BTC/USD", "ETH/USD", "SOL/USD"];
    let duration_secs = 30;

    println!("⚙️  Configuration:");
    println!("   Pairs: {}", pairs.join(", "));
    println!("   Duration: {} seconds", duration_secs);
    println!("   Export: Orderbook snapshots per pair\n");

    // Create timestamped filenames
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

    // Initialize exporters for each pair
    let mut exporters: HashMap<String, PairExporter> = HashMap::new();

    println!("📁 Creating CSV files:");
    for pair in &pairs {
        let exporter = PairExporter::new(pair, &timestamp.to_string())?;
        let pair_filename = pair.replace('/', "");
        println!("   ✅ {}: orderbook_{}_{}.csv", pair, pair_filename, timestamp);
        exporters.insert(pair.to_string(), exporter);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Connect to Kraken
    // ═══════════════════════════════════════════════════════════════════════

    println!("📡 Connecting to Kraken WebSocket...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Subscribe to All Pairs
    // ═══════════════════════════════════════════════════════════════════════

    println!("📊 Subscribing to market data...");

    // Subscribe to orderbooks
    for pair in &pairs {
        client.subscribe_orderbook(pair, 10).await?;
        println!("   ✅ Orderbook: {}", pair);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 4: Stream and Export Data
    // ═══════════════════════════════════════════════════════════════════════

    println!("🚀 Streaming data to CSV files...");
    println!("   Press Ctrl+C to stop\n");
    println!("{}", "═".repeat(70));

    let start_time = tokio::time::Instant::now();
    let duration = tokio::time::Duration::from_secs(duration_secs);

    let mut total_orderbook = 0;

    loop {
        // Check if duration exceeded
        if start_time.elapsed() >= duration {
            println!("\n{}", "═".repeat(70));
            println!("⏰ Duration limit reached");
            break;
        }

        // Process orderbook updates for all pairs
        for pair in &pairs {
            if let Some(ob) = client.get_orderbook(pair) {
                if let Some(exporter) = exporters.get_mut(*pair) {
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
                        exporter.orderbook_file,
                        "{},{},{},{},{},{},{:.4},{:.4},{:.4}",
                        timestamp,
                        pair,
                        best_bid,
                        best_ask,
                        spread,
                        mid_price,
                        imbalance,
                        bid_volume,
                        ask_volume
                    )?;

                    exporter.orderbook_count += 1;
                    total_orderbook += 1;
                }
            }
        }

        // Update progress display every 50 records
        if total_orderbook % 50 == 0 {
            print!("\r📖 Total: {} orderbook snapshots | ", total_orderbook);

            // Show per-pair counts
            for pair in &pairs {
                if let Some(exporter) = exporters.get(*pair) {
                    print!("{}: {} | ", pair.split('/').next().unwrap(), exporter.orderbook_count);
                }
            }
            std::io::stdout().flush()?;
        }

        // Small delay to prevent busy-waiting
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    println!("\n{}", "═".repeat(70));

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 5: Finalize and Summary
    // ═══════════════════════════════════════════════════════════════════════

    // Flush all files
    for (_, exporter) in exporters.iter_mut() {
        exporter.orderbook_file.flush()?;
    }

    println!("\n✅ Export completed!\n");

    println!("📊 Summary by Pair:");
    for pair in &pairs {
        if let Some(exporter) = exporters.get(*pair) {
            println!("   {}: {} orderbook snapshots", pair, exporter.orderbook_count);
        }
    }
    println!();

    println!("📊 Total Summary:");
    println!("   Total orderbook records: {}", total_orderbook);
    println!("   Duration: {:.1}s", start_time.elapsed().as_secs_f64());
    println!("   Files created: {} CSV files", pairs.len());
    println!();

    println!("📁 Files Created:");
    for pair in &pairs {
        let pair_filename = pair.replace('/', "");
        println!("   orderbook_{}_{}.csv", pair_filename, timestamp);
    }
    println!();

    println!("💡 Next Steps:");
    println!("   - Compare price movements across pairs");
    println!("   - Analyze correlation between assets");
    println!("   - Build multi-asset trading strategies");
    println!("   - Create cross-pair arbitrage datasets");
    println!();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              📊 Multi-Pair Export Complete! 🎉               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    Ok(())
}
