//! Kraky SDK Demo for Hackathon Judges
//!
//! This example demonstrates all key features of the SDK in one place.
//! Run with: cargo run --example demo
//!
//! Features demonstrated:
//! - WebSocket connection to Kraken
//! - Orderbook subscription with managed state
//! - **Orderbook imbalance detection** (bullish/bearish signals)
//! - Trade subscription
//! - Ticker subscription
//! - Backpressure monitoring
//! - Error handling

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
    println!("║           🐙 KRAKY SDK DEMO - Kraken Forge Hackathon         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 1: Connect to Kraken WebSocket
    // ═══════════════════════════════════════════════════════════════════════
    println!("📡 Connecting to Kraken WebSocket API...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 2: Subscribe to multiple data streams
    // ═══════════════════════════════════════════════════════════════════════
    println!("📊 Subscribing to BTC/USD data streams...\n");

    let mut orderbook_sub = client.subscribe_orderbook("BTC/USD", 10).await?;
    let mut trades_sub = client.subscribe_trades("BTC/USD").await?;
    let mut ticker_sub = client.subscribe_ticker("BTC/USD").await?;

    // Give subscriptions time to initialize
    tokio::time::sleep(Duration::from_secs(2)).await;

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 3: Display real-time data
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    LIVE MARKET DATA");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut orderbook_count = 0;
    let mut trade_count = 0;
    let mut ticker_count = 0;
    let demo_duration = Duration::from_secs(15);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() > demo_duration {
            break;
        }

        tokio::select! {
            Some(_update) = orderbook_sub.next() => {
                orderbook_count += 1;
                if orderbook_count <= 3 {
                    println!("📖 ORDERBOOK UPDATE #{}", orderbook_count);
                    
                    // Show managed state
                    if let Some(ob) = client.get_orderbook("BTC/USD") {
                        if let (Some(bid), Some(ask)) = (ob.best_bid(), ob.best_ask()) {
                            println!("   Best Bid: ${:.2}", bid);
                            println!("   Best Ask: ${:.2}", ask);
                            if let Some(spread) = ob.spread() {
                                println!("   Spread:   ${:.2}", spread);
                            }
                            if let Some(mid) = ob.mid_price() {
                                println!("   Mid:      ${:.2}", mid);
                            }
                        }
                        
                        // Show top levels
                        let bids = ob.top_bids(3);
                        let asks = ob.top_asks(3);
                        println!("   Top 3 Bids: {:?}", bids.iter().map(|l| format!("${:.0}", l.price)).collect::<Vec<_>>());
                        println!("   Top 3 Asks: {:?}", asks.iter().map(|l| format!("${:.0}", l.price)).collect::<Vec<_>>());
                    }
                    
                    println!();
                }
            }
            
            Some(trade) = trades_sub.next() => {
                trade_count += 1;
                if trade_count <= 10 {
                    let side_emoji = if format!("{:?}", trade.side).contains("Buy") { "🟢" } else { "🔴" };
                    println!("{} TRADE: {:?} {:.6} BTC @ ${:.2}", 
                        side_emoji, trade.side, trade.qty, trade.price);
                }
            }
            
            Some(tick) = ticker_sub.next() => {
                ticker_count += 1;
                if ticker_count <= 5 {
                    println!("📈 TICKER: ${:.2} (24h: {:+.2}%) Vol: {:.2} BTC", 
                        tick.last, tick.change_pct, tick.volume);
                }
            }
            
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // Periodic timeout to check demo duration
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 4: Show statistics
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    DEMO STATISTICS");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Messages received in {} seconds:", demo_duration.as_secs());
    println!("   📖 Orderbook updates: {}", orderbook_count);
    println!("   💱 Trades:            {}", trade_count);
    println!("   📈 Ticker updates:    {}", ticker_count);
    println!();

    // Show backpressure stats
    let ob_stats = orderbook_sub.stats();
    let trade_stats = trades_sub.stats();
    let ticker_stats = ticker_sub.stats();

    println!("Backpressure stats (delivered / dropped / drop rate):");
    println!("   📖 Orderbook: {} / {} / {:.2}%", 
        ob_stats.delivered(), ob_stats.dropped(), ob_stats.drop_rate());
    println!("   💱 Trades:    {} / {} / {:.2}%", 
        trade_stats.delivered(), trade_stats.dropped(), trade_stats.drop_rate());
    println!("   📈 Ticker:    {} / {} / {:.2}%", 
        ticker_stats.delivered(), ticker_stats.dropped(), ticker_stats.drop_rate());

    // ═══════════════════════════════════════════════════════════════════════
    // STEP 5: Final orderbook state
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    FINAL ORDERBOOK STATE");
    println!("═══════════════════════════════════════════════════════════════\n");

    if let Some(ob) = client.get_orderbook("BTC/USD") {
        println!("BTC/USD Orderbook (after {} updates):", orderbook_count);
        println!("   Bid levels: {}", ob.bids.len());
        println!("   Ask levels: {}", ob.asks.len());
        
        if let (Some(bid), Some(ask)) = (ob.best_bid(), ob.best_ask()) {
            println!("\n   ┌─────────────────────────────────────┐");
            println!("   │  Best Bid: ${:<22.2} │", bid);
            println!("   │  Best Ask: ${:<22.2} │", ask);
            if let Some(spread) = ob.spread() {
                println!("   │  Spread:   ${:<22.2} │", spread);
            }
            if let Some(mid) = ob.mid_price() {
                println!("   │  Mid Price: ${:<21.2} │", mid);
            }
            println!("   └─────────────────────────────────────┘");
        }

        // Show orderbook imbalance analysis
        println!("\n═══════════════════════════════════════════════════════════════");
        println!("                    ORDERBOOK IMBALANCE");
        println!("═══════════════════════════════════════════════════════════════\n");

        let metrics = ob.imbalance_metrics();
        let signal = metrics.signal(0.1);
        let signal_emoji = match signal {
            kraky::ImbalanceSignal::Bullish => "🟢 BULLISH",
            kraky::ImbalanceSignal::Bearish => "🔴 BEARISH",
            kraky::ImbalanceSignal::Neutral => "⚪ NEUTRAL",
        };

        println!("   Full Orderbook Analysis:");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │  Bid Volume:   {:<18.4} BTC │", metrics.bid_volume);
        println!("   │  Ask Volume:   {:<18.4} BTC │", metrics.ask_volume);
        println!("   │  Bid/Ask Ratio: {:<17.4}   │", metrics.bid_ask_ratio);
        println!("   │  Imbalance:     {:>+17.2}%   │", metrics.imbalance_ratio * 100.0);
        println!("   │  Signal:       {:<18}  │", signal_emoji);
        println!("   └─────────────────────────────────────┘");

        // Top-of-book imbalance (most actionable)
        let top5_imbalance = ob.imbalance_top_n(5);
        let top5_signal = if top5_imbalance > 0.1 { "🟢" } else if top5_imbalance < -0.1 { "🔴" } else { "⚪" };
        println!("\n   Top 5 Levels Imbalance: {:>+.2}% {}", top5_imbalance * 100.0, top5_signal);
        
        // Imbalance within 0.5% of mid price (tight spread)
        if let Some(tight_imbalance) = ob.imbalance_within_depth(0.005) {
            let tight_signal = if tight_imbalance > 0.1 { "🟢" } else if tight_imbalance < -0.1 { "🔴" } else { "⚪" };
            println!("   Within 0.5% of Mid:     {:>+.2}% {}", tight_imbalance * 100.0, tight_signal);
        }
    }

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    🎉 DEMO COMPLETE!                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Disconnect cleanly
    client.disconnect();

    Ok(())
}

