//! Kraky SDK Benchmark
//!
//! Measures latency and throughput of the SDK.
//!
//! ## Run
//! ```bash
//! cargo run --example benchmark --features trades,ticker --release
//! ```
//!
//! ## Metrics tracked
//! - Connection time
//! - Message processing latency
//! - Messages per second throughput
//! - Orderbook update latency

use kraky::KrakyClient;
use std::time::{Duration, Instant};

#[derive(Default)]
struct BenchmarkStats {
    // Connection
    connection_time_ms: f64,

    // Message counts
    orderbook_messages: u64,
    trade_messages: u64,
    ticker_messages: u64,

    // Latency tracking (in microseconds)
    latencies_us: Vec<u64>,

    // Timing
    first_message_time_ms: Option<f64>,
    total_duration_ms: f64,
}

impl BenchmarkStats {
    fn add_latency(&mut self, latency_us: u64) {
        self.latencies_us.push(latency_us);
    }

    fn percentile(&self, p: f64) -> u64 {
        if self.latencies_us.is_empty() {
            return 0;
        }
        let mut sorted = self.latencies_us.clone();
        sorted.sort();
        let idx = ((p / 100.0) * sorted.len() as f64) as usize;
        sorted[idx.min(sorted.len() - 1)]
    }

    fn mean_latency(&self) -> f64 {
        if self.latencies_us.is_empty() {
            return 0.0;
        }
        self.latencies_us.iter().sum::<u64>() as f64 / self.latencies_us.len() as f64
    }

    fn min_latency(&self) -> u64 {
        self.latencies_us.iter().copied().min().unwrap_or(0)
    }

    fn max_latency(&self) -> u64 {
        self.latencies_us.iter().copied().max().unwrap_or(0)
    }

    fn total_messages(&self) -> u64 {
        self.orderbook_messages + self.trade_messages + self.ticker_messages
    }

    fn messages_per_second(&self) -> f64 {
        if self.total_duration_ms == 0.0 {
            return 0.0;
        }
        (self.total_messages() as f64) / (self.total_duration_ms / 1000.0)
    }

    fn print_report(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                 🐙 KRAKY SDK BENCHMARK RESULTS                ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("📡 CONNECTION");
        println!("   Connection time:      {:.2} ms", self.connection_time_ms);
        if let Some(first_msg) = self.first_message_time_ms {
            println!("   Time to first message: {:.2} ms", first_msg);
        }
        println!();

        println!("📊 THROUGHPUT");
        println!("   Total messages:       {}", self.total_messages());
        println!("   - Orderbook updates:  {}", self.orderbook_messages);
        println!("   - Trade updates:      {}", self.trade_messages);
        println!("   - Ticker updates:     {}", self.ticker_messages);
        println!(
            "   Duration:             {:.2} seconds",
            self.total_duration_ms / 1000.0
        );
        println!("   Messages/second:      {:.2}", self.messages_per_second());
        println!();

        println!("⏱️  LATENCY (message processing time)");
        println!("   Samples:              {}", self.latencies_us.len());
        println!("   Mean:                 {:.2} µs", self.mean_latency());
        println!("   Min:                  {} µs", self.min_latency());
        println!("   Max:                  {} µs", self.max_latency());
        println!("   P50 (median):         {} µs", self.percentile(50.0));
        println!("   P95:                  {} µs", self.percentile(95.0));
        println!("   P99:                  {} µs", self.percentile(99.0));
        println!("   P99.9:                {} µs", self.percentile(99.9));
        println!();

        // Performance rating
        let mean = self.mean_latency();
        let rating = if mean < 10.0 {
            "🚀 EXCELLENT (<10µs)"
        } else if mean < 50.0 {
            "✅ GREAT (<50µs)"
        } else if mean < 100.0 {
            "👍 GOOD (<100µs)"
        } else if mean < 500.0 {
            "⚠️  ACCEPTABLE (<500µs)"
        } else {
            "🐢 NEEDS IMPROVEMENT (>500µs)"
        };
        println!("📈 PERFORMANCE RATING: {}", rating);
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize minimal logging
    tracing_subscriber::fmt()
        .with_env_filter("warn")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                 🐙 KRAKY SDK BENCHMARK                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut stats = BenchmarkStats::default();

    // Benchmark duration
    let benchmark_duration = Duration::from_secs(10);

    println!(
        "⏳ Running benchmark for {} seconds...\n",
        benchmark_duration.as_secs()
    );
    println!("   Subscribing to: BTC/USD orderbook, trades, ticker");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // Measure connection time
    // ═══════════════════════════════════════════════════════════════════════
    let connect_start = Instant::now();
    let client = KrakyClient::connect().await?;
    stats.connection_time_ms = connect_start.elapsed().as_secs_f64() * 1000.0;

    println!("✅ Connected in {:.2} ms", stats.connection_time_ms);

    // ═══════════════════════════════════════════════════════════════════════
    // Subscribe to streams
    // ═══════════════════════════════════════════════════════════════════════
    let mut orderbook_sub = client.subscribe_orderbook("BTC/USD", 10).await?;
    let mut trades_sub = client.subscribe_trades("BTC/USD").await?;
    let mut ticker_sub = client.subscribe_ticker("BTC/USD").await?;

    // Wait for subscriptions to be ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("✅ Subscriptions active");
    println!();
    println!("📊 Collecting data...");

    // ═══════════════════════════════════════════════════════════════════════
    // Benchmark loop
    // ═══════════════════════════════════════════════════════════════════════
    let benchmark_start = Instant::now();
    let mut first_message_received = false;
    let mut progress_counter = 0u64;

    loop {
        if benchmark_start.elapsed() > benchmark_duration {
            break;
        }

        let loop_start = Instant::now();

        tokio::select! {
            Some(_update) = orderbook_sub.next() => {
                let latency_us = loop_start.elapsed().as_micros() as u64;
                stats.add_latency(latency_us);
                stats.orderbook_messages += 1;

                if !first_message_received {
                    stats.first_message_time_ms = Some(benchmark_start.elapsed().as_secs_f64() * 1000.0);
                    first_message_received = true;
                }
            }

            Some(_trade) = trades_sub.next() => {
                let latency_us = loop_start.elapsed().as_micros() as u64;
                stats.add_latency(latency_us);
                stats.trade_messages += 1;

                if !first_message_received {
                    stats.first_message_time_ms = Some(benchmark_start.elapsed().as_secs_f64() * 1000.0);
                    first_message_received = true;
                }
            }

            Some(_tick) = ticker_sub.next() => {
                let latency_us = loop_start.elapsed().as_micros() as u64;
                stats.add_latency(latency_us);
                stats.ticker_messages += 1;

                if !first_message_received {
                    stats.first_message_time_ms = Some(benchmark_start.elapsed().as_secs_f64() * 1000.0);
                    first_message_received = true;
                }
            }

            _ = tokio::time::sleep(Duration::from_millis(1)) => {
                // Timeout to check duration
            }
        }

        // Progress indicator
        progress_counter += 1;
        if progress_counter % 100 == 0 {
            let elapsed = benchmark_start.elapsed().as_secs();
            let remaining = benchmark_duration.as_secs().saturating_sub(elapsed);
            print!(
                "\r   {} messages received, {} seconds remaining...   ",
                stats.total_messages(),
                remaining
            );
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }

    stats.total_duration_ms = benchmark_start.elapsed().as_secs_f64() * 1000.0;

    println!("\r                                                              ");

    // Disconnect
    client.disconnect();

    // ═══════════════════════════════════════════════════════════════════════
    // Print results
    // ═══════════════════════════════════════════════════════════════════════
    stats.print_report();

    // Save results to file for comparison
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("benchmark_{}.txt", timestamp);

    let report = format!(
        "Kraky SDK Benchmark - {}\n\
         Connection: {:.2}ms\n\
         Messages: {} total ({:.2}/sec)\n\
         Latency: mean={:.2}µs, p50={}µs, p99={}µs, max={}µs\n",
        timestamp,
        stats.connection_time_ms,
        stats.total_messages(),
        stats.messages_per_second(),
        stats.mean_latency(),
        stats.percentile(50.0),
        stats.percentile(99.0),
        stats.max_latency()
    );

    if std::fs::write(&filename, &report).is_ok() {
        println!("💾 Results saved to: {}", filename);
    }

    println!("\n💡 TIP: Run with --release for accurate results:");
    println!("   cargo run --example benchmark --release\n");

    Ok(())
}
