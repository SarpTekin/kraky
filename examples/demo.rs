//! Kraky SDK Demo for Hackathon Judges
//!
//! This example demonstrates ALL key features of the SDK in one place.
//! Run with: cargo run --example demo
//!
//! Features demonstrated:
//! ✅ WebSocket connection to Kraken
//! ✅ Connection events (connect/disconnect/reconnect callbacks)
//! ✅ Connection state monitoring
//! ✅ Orderbook subscription with managed state
//! ✅ Orderbook imbalance detection (bullish/bearish signals)
//! ✅ Orderbook checksum validation
//! ✅ Trade subscription
//! ✅ Ticker subscription
//! ✅ Backpressure monitoring
//! ✅ Smart reconnection configuration

use kraky::{KrakyClient, ConnectionEvent, ConnectionState, ImbalanceSignal};
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
    // FEATURE 1: Connect to Kraken WebSocket
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 1: WebSocket Connection");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("📡 Connecting to Kraken WebSocket API...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected!\n");

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 2: Connection Events (lifecycle callbacks)
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 2: Connection Events");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut events = client.subscribe_events();
    println!("📌 Subscribed to connection events");
    println!("   Events: Connected, Disconnected, Reconnecting, Reconnected,");
    println!("           ReconnectFailed, ReconnectExhausted\n");

    // Spawn event handler in background
    tokio::spawn(async move {
        while let Some(event) = events.recv().await {
            match event {
                ConnectionEvent::Connected => println!("🔔 EVENT: Connected"),
                ConnectionEvent::Disconnected(reason) => {
                    println!("🔔 EVENT: Disconnected - {:?}", reason)
                }
                ConnectionEvent::Reconnecting(n) => {
                    println!("🔔 EVENT: Reconnecting (attempt #{})", n)
                }
                ConnectionEvent::Reconnected => println!("🔔 EVENT: Reconnected"),
                ConnectionEvent::ReconnectFailed(n, e) => {
                    println!("🔔 EVENT: Reconnect failed #{}: {}", n, e)
                }
                ConnectionEvent::ReconnectExhausted => {
                    println!("🔔 EVENT: Reconnect exhausted")
                }
            }
        }
    });

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 3: Connection State Monitoring
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 3: Connection State");
    println!("═══════════════════════════════════════════════════════════════\n");

    let state = client.connection_state();
    let state_str = match state {
        ConnectionState::Connected => "✅ Connected",
        ConnectionState::Connecting => "🔄 Connecting",
        ConnectionState::Reconnecting => "🔄 Reconnecting",
        ConnectionState::Disconnected => "❌ Disconnected",
    };
    println!("   Current state: {}", state_str);
    println!("   is_connected(): {}", client.is_connected());
    println!("   is_reconnecting(): {}", client.is_reconnecting());
    println!("   URL: {}\n", client.url());

    // Show reconnect config
    let config = client.reconnect_config();
    println!("   Reconnect Config:");
    println!("     - Enabled: {}", config.enabled);
    println!("     - Initial delay: {:?}", config.initial_delay);
    println!("     - Max delay: {:?}", config.max_delay);
    println!("     - Backoff multiplier: {}x", config.backoff_multiplier);
    println!("     - Max attempts: {:?}\n", config.max_attempts);

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 4: Subscribe to Multiple Data Streams
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 4: Data Subscriptions");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("📊 Subscribing to BTC/USD data streams...");
    let mut orderbook_sub = client.subscribe_orderbook("BTC/USD", 10).await?;
    println!("   ✅ Orderbook (depth: 10)");
    
    let mut trades_sub = client.subscribe_trades("BTC/USD").await?;
    println!("   ✅ Trades");
    
    let mut ticker_sub = client.subscribe_ticker("BTC/USD").await?;
    println!("   ✅ Ticker\n");

    // Give subscriptions time to initialize
    tokio::time::sleep(Duration::from_secs(2)).await;

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 5: Real-time Market Data
    // ═══════════════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 5: Live Market Data (15 seconds)");
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
                    
                    if let Some(ob) = client.get_orderbook("BTC/USD") {
                        if let (Some(bid), Some(ask)) = (ob.best_bid(), ob.best_ask()) {
                            println!("   Best Bid: ${:.2} | Best Ask: ${:.2}", bid, ask);
                            if let Some(spread) = ob.spread() {
                                println!("   Spread: ${:.2} | Mid: ${:.2}", spread, ob.mid_price().unwrap_or(0.0));
                            }
                        }
                        
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
            
            _ = tokio::time::sleep(Duration::from_millis(100)) => {}
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 6: Backpressure Monitoring
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 6: Backpressure Monitoring");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Messages received in {} seconds:", demo_duration.as_secs());
    println!("   📖 Orderbook: {}", orderbook_count);
    println!("   💱 Trades:    {}", trade_count);
    println!("   📈 Ticker:    {}", ticker_count);
    println!();

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
    // FEATURE 7: Orderbook Checksum Validation (requires 'checksum' feature)
    // ═══════════════════════════════════════════════════════════════════════
    #[cfg(feature = "checksum")]
    {
        println!("\n═══════════════════════════════════════════════════════════════");
        println!("  FEATURE 7: Orderbook Checksum Validation (CRC32)");
        println!("═══════════════════════════════════════════════════════════════\n");

        if let Some(ob) = client.get_orderbook("BTC/USD") {
            let checksum = ob.calculate_checksum();
            println!("   Calculated Checksum: 0x{:08X}", checksum);
            println!("   Last Checksum:       0x{:08X}", ob.last_checksum);
            println!("   Checksum Valid:      {}", if ob.checksum_valid { "✅ Yes" } else { "❌ No" });
            
            // Show validation helper
            let is_valid = client.is_orderbook_valid("BTC/USD");
            println!("   is_orderbook_valid(): {:?}", is_valid);
        }
    }
    
    #[cfg(not(feature = "checksum"))]
    {
        println!("\n═══════════════════════════════════════════════════════════════");
        println!("  FEATURE 7: Orderbook Checksum Validation");
        println!("═══════════════════════════════════════════════════════════════\n");
        println!("   ⚠️  Checksum feature not enabled.");
        println!("   Enable with: cargo run --example demo --features checksum");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 8: Orderbook Imbalance Detection
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 8: Orderbook Imbalance Detection");
    println!("═══════════════════════════════════════════════════════════════\n");

    if let Some(ob) = client.get_orderbook("BTC/USD") {
        let metrics = ob.imbalance_metrics();
        let signal = metrics.signal(0.1);
        let signal_str = match signal {
            ImbalanceSignal::Bullish => "🟢 BULLISH (more buy pressure)",
            ImbalanceSignal::Bearish => "🔴 BEARISH (more sell pressure)",
            ImbalanceSignal::Neutral => "⚪ NEUTRAL (balanced)",
        };

        println!("   Full Orderbook Analysis:");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │  Bid Volume:   {:<18.4} BTC │", metrics.bid_volume);
        println!("   │  Ask Volume:   {:<18.4} BTC │", metrics.ask_volume);
        println!("   │  Bid/Ask Ratio: {:<17.4}   │", metrics.bid_ask_ratio);
        println!("   │  Imbalance:     {:>+17.2}%   │", metrics.imbalance_ratio * 100.0);
        println!("   │  Signal:       {:<18}  │", signal_str.split(" (").next().unwrap());
        println!("   └─────────────────────────────────────┘");
        println!();
        println!("   Signal Interpretation: {}", signal_str);

        // Different imbalance calculations
        let full = ob.imbalance();
        let top5 = ob.imbalance_top_n(5);
        let tight = ob.imbalance_within_depth(0.005);

        println!();
        println!("   Imbalance Methods:");
        println!("     - Full orderbook:  {:>+.2}%", full * 100.0);
        println!("     - Top 5 levels:    {:>+.2}%", top5 * 100.0);
        if let Some(t) = tight {
            println!("     - Within 0.5%:     {:>+.2}%", t * 100.0);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FEATURE 9: Managed Orderbook State
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  FEATURE 9: Managed Orderbook State");
    println!("═══════════════════════════════════════════════════════════════\n");

    if let Some(ob) = client.get_orderbook("BTC/USD") {
        println!("   Symbol: {}", ob.symbol);
        println!("   Sequence: {}", ob.sequence);
        println!("   Bid levels: {}", ob.bids.len());
        println!("   Ask levels: {}", ob.asks.len());
        
        if let (Some(bid), Some(ask)) = (ob.best_bid(), ob.best_ask()) {
            println!();
            println!("   ┌─────────────────────────────────────┐");
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
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SUMMARY
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    🎉 DEMO COMPLETE!                          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Features Demonstrated:                                       ║");
    println!("║    ✅ WebSocket Connection                                    ║");
    println!("║    ✅ Connection Events (lifecycle callbacks)                 ║");
    println!("║    ✅ Connection State Monitoring                             ║");
    println!("║    ✅ Multiple Subscriptions (orderbook, trades, ticker)      ║");
    println!("║    ✅ Real-time Market Data Processing                        ║");
    println!("║    ✅ Backpressure Monitoring                                 ║");
    println!("║    ✅ Orderbook Checksum Validation (CRC32)                   ║");
    println!("║    ✅ Orderbook Imbalance Detection                           ║");
    println!("║    ✅ Managed Orderbook State                                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Disconnect cleanly
    client.disconnect();

    Ok(())
}
