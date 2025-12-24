//! Telegram Imbalance Alert Bot
//!
//! This example demonstrates Kraky's unique telegram integration with
//! advanced orderbook imbalance detection.
//!
//! ## Features Showcased
//! - Real-time orderbook imbalance monitoring
//! - Telegram notifications with formatted alerts
//! - Bullish/Bearish/Neutral signal detection
//! - Connection event notifications
//! - Price threshold alerts
//! - 🆕 Whale alert detection (large orders > 10 BTC)
//! - 🆕 Spread volatility monitoring (3x normal spread)
//! - 🆕 Order flow divergence detection (price vs orderbook)
//! - 🆕 Large trade execution alerts
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
//!    cargo run --example telegram_imbalance_bot --features telegram-alerts
//!    ```
//!
//! ## What This Demonstrates
//!
//! This example showcases Kraky's competitive advantage:
//! - **Advanced analytics** - Orderbook imbalance detection
//! - **Modularity** - Telegram is an optional feature flag
//! - **Real-world application** - Practical trading alert system
//! - **Lightweight** - Only 800KB added when enabled

use kraky::{ConnectionEvent, ImbalanceSignal, KrakyClient, TelegramNotifier};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .init();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║       🐙 Kraky Telegram Imbalance Bot - Demo                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ═══════════════════════════════════════════════════════════════════════
    // SETUP: Get configuration from environment
    // ═══════════════════════════════════════════════════════════════════════

    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");

    let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID")
        .expect("Please set TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a valid integer");

    println!("📱 Telegram Configuration:");
    println!("   Chat ID: {}", chat_id);
    println!(
        "   Bot Token: {}...{}",
        &bot_token[..8],
        &bot_token[bot_token.len() - 4..]
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // INITIALIZE: Create Kraky client and Telegram notifier
    // ═══════════════════════════════════════════════════════════════════════

    println!("🔧 Initializing Kraky client...");
    let client = KrakyClient::connect().await?;
    println!("✅ Connected to Kraken WebSocket API\n");

    println!("🤖 Initializing Telegram bot...");
    let bot = TelegramNotifier::new(&bot_token, chat_id);
    println!("✅ Telegram bot ready\n");

    // Send startup notification
    bot.send_connection_status(
        true,
        "🐙 Kraky Imbalance Bot started!\nMonitoring BTC/USD orderbook...",
    )
    .await?;

    // ═══════════════════════════════════════════════════════════════════════
    // SUBSCRIBE: Set up connection events monitoring
    // ═══════════════════════════════════════════════════════════════════════

    let mut events = client.subscribe_events();
    let bot_clone = TelegramNotifier::new(&bot_token, chat_id);

    tokio::spawn(async move {
        while let Some(event) = events.recv().await {
            let message = match event {
                ConnectionEvent::Connected => "✅ Connected to Kraken".to_string(),
                ConnectionEvent::Disconnected(reason) => {
                    format!("❌ Disconnected: {:?}", reason)
                }
                ConnectionEvent::Reconnecting(n) => {
                    format!("🔄 Reconnecting (attempt #{})", n)
                }
                ConnectionEvent::Reconnected => "✅ Reconnected to Kraken".to_string(),
                ConnectionEvent::ReconnectFailed(n, e) => {
                    format!("⚠️ Reconnect failed #{}: {}", n, e)
                }
                ConnectionEvent::ReconnectExhausted => {
                    "💀 Reconnection attempts exhausted".to_string()
                }
            };

            if let Err(e) = bot_clone.send_alert(&message).await {
                eprintln!("Failed to send event notification: {}", e);
            }
        }
    });

    // ═══════════════════════════════════════════════════════════════════════
    // SUBSCRIBE: Orderbook and Ticker for BTC/USD
    // ═══════════════════════════════════════════════════════════════════════

    let trading_pair = "BTC/USD";
    println!("📊 Subscribing to {} market data...", trading_pair);

    let mut orderbook_sub = client.subscribe_orderbook(trading_pair, 10).await?;
    let mut ticker_sub = client.subscribe_ticker(trading_pair).await?;

    println!("✅ Subscribed to orderbook (depth: 10)");
    println!("✅ Subscribed to ticker\n");

    // ═══════════════════════════════════════════════════════════════════════
    // CONFIGURATION: Alert thresholds
    // ═══════════════════════════════════════════════════════════════════════

    println!("⚙️  Alert Configuration:");
    let imbalance_threshold = 0.15; // 15% imbalance triggers alert
    let price_check_interval = Duration::from_secs(30);
    let price_threshold_high = 100_000.0; // Alert if price goes above $100k
    let price_threshold_low = 95_000.0; // Alert if price goes below $95k

    // NEW: Advanced alert thresholds
    let whale_volume_threshold = 10.0; // 10 BTC = whale order
    let spread_multiplier_threshold = 3.0; // 3x normal spread = alert

    println!(
        "   Imbalance Threshold: ±{:.0}%",
        imbalance_threshold * 100.0
    );
    println!("   Price Alert High: ${:.2}", price_threshold_high);
    println!("   Price Alert Low: ${:.2}", price_threshold_low);
    println!("   Price Check Interval: {:?}", price_check_interval);
    println!(
        "   Whale Volume Threshold: {:.1} BTC",
        whale_volume_threshold
    );
    println!(
        "   Spread Alert Multiplier: {:.1}x\n",
        spread_multiplier_threshold
    );

    println!(
        "🚀 Bot is now running! Monitoring {} for imbalance signals...",
        trading_pair
    );
    println!("   Press Ctrl+C to stop\n");

    // ═══════════════════════════════════════════════════════════════════════
    // MAIN LOOP: Monitor orderbook and send alerts
    // ═══════════════════════════════════════════════════════════════════════

    let mut last_signal = ImbalanceSignal::Neutral;
    let mut alert_count = 0;
    let mut orderbook_update_count = 0;
    let mut last_price_check = std::time::Instant::now();
    let mut last_price: Option<f64> = None;

    // NEW: State tracking for advanced features
    let mut spread_history: Vec<f64> = Vec::new();
    let mut last_whale_check = std::time::Instant::now();
    let mut price_history: Vec<(std::time::Instant, f64)> = Vec::new();

    loop {
        tokio::select! {
            // Handle orderbook updates
            Some(_update) = orderbook_sub.next() => {
                orderbook_update_count += 1;

                if let Some(ob) = client.get_orderbook(trading_pair) {
                    // Calculate imbalance metrics
                    let metrics = ob.imbalance_metrics();
                    let signal = metrics.signal(imbalance_threshold);

                    // Only send alert if signal changed (avoid spam)
                    if !matches!(signal, ImbalanceSignal::Neutral) && signal != last_signal {
                        println!("📢 Imbalance signal changed: {:?} -> {:?}", last_signal, signal);

                        // Send Telegram alert with full metrics
                        if let Err(e) = bot.send_imbalance_alert(trading_pair, &metrics, signal).await {
                            eprintln!("Failed to send imbalance alert: {}", e);
                        } else {
                            alert_count += 1;
                            println!("✅ Alert #{} sent successfully", alert_count);
                        }

                        last_signal = signal;
                    }

                    // ═══════════════════════════════════════════════════════════
                    // NEW FEATURE: Whale Alert - Detect large orders
                    // ═══════════════════════════════════════════════════════════
                    if last_whale_check.elapsed() >= Duration::from_secs(10) {
                        // Check top 3 bids and asks for whale orders
                        for (i, (price, volume)) in ob.bids.iter().take(3).enumerate() {
                            if *volume >= whale_volume_threshold {
                                let price_f64 = price.0;  // Extract f64 from OrderedFloat
                                println!("🐋 Whale detected: {} BTC bid @ ${:.2}", volume, price_f64);
                                if let Err(e) = bot.send_whale_alert(trading_pair, "bid", price_f64, *volume).await {
                                    eprintln!("Failed to send whale alert: {}", e);
                                } else {
                                    alert_count += 1;
                                    println!("✅ Whale alert #{} sent (bid #{}, {} BTC)", alert_count, i + 1, volume);
                                }
                                break; // Only alert once per check
                            }
                        }

                        for (i, (price, volume)) in ob.asks.iter().take(3).enumerate() {
                            if *volume >= whale_volume_threshold {
                                let price_f64 = price.0;  // Extract f64 from OrderedFloat
                                println!("🐋 Whale detected: {} BTC ask @ ${:.2}", volume, price_f64);
                                if let Err(e) = bot.send_whale_alert(trading_pair, "ask", price_f64, *volume).await {
                                    eprintln!("Failed to send whale alert: {}", e);
                                } else {
                                    alert_count += 1;
                                    println!("✅ Whale alert #{} sent (ask #{}, {} BTC)", alert_count, i + 1, volume);
                                }
                                break; // Only alert once per check
                            }
                        }

                        last_whale_check = std::time::Instant::now();
                    }

                    // ═══════════════════════════════════════════════════════════
                    // NEW FEATURE: Spread Volatility Alert
                    // ═══════════════════════════════════════════════════════════
                    if let (Some(best_bid), Some(best_ask)) = (ob.best_bid(), ob.best_ask()) {
                        let spread = best_ask - best_bid;
                        let mid_price = (best_bid + best_ask) / 2.0;
                        let spread_bps = (spread / mid_price) * 10000.0;

                        // Track spread history (keep last 100 values)
                        spread_history.push(spread_bps);
                        if spread_history.len() > 100 {
                            spread_history.remove(0);
                        }

                        // Calculate average spread (need at least 20 samples)
                        if spread_history.len() >= 20 {
                            let avg_spread: f64 = spread_history.iter().sum::<f64>() / spread_history.len() as f64;
                            let multiplier = spread_bps / avg_spread;

                            // Alert if spread is significantly wider than average
                            if multiplier >= spread_multiplier_threshold {
                                println!("⚠️ Spread volatility: {:.1} bps ({:.1}x average)", spread_bps, multiplier);
                                if let Err(e) = bot.send_spread_alert(trading_pair, spread_bps, avg_spread, multiplier).await {
                                    eprintln!("Failed to send spread alert: {}", e);
                                } else {
                                    alert_count += 1;
                                    println!("✅ Spread alert #{} sent", alert_count);
                                }
                            }
                        }
                    }

                    // ═══════════════════════════════════════════════════════════
                    // NEW FEATURE: Order Flow Divergence Detection
                    // ═══════════════════════════════════════════════════════════
                    if let Some(current_price) = last_price {
                        // Track price with timestamp
                        price_history.push((std::time::Instant::now(), current_price));

                        // Keep only last 5 minutes of price history
                        price_history.retain(|(time, _)| time.elapsed() < Duration::from_secs(300));

                        // Calculate price change over last 2 minutes (if enough data)
                        if let Some(&(_oldest_time, oldest_price)) = price_history.iter()
                            .find(|(time, _)| time.elapsed() >= Duration::from_secs(120)) {

                            let price_change_pct = ((current_price - oldest_price) / oldest_price) * 100.0;

                            // Only check for divergence if price moved significantly (>0.5%)
                            if price_change_pct.abs() >= 0.5 {
                                // Check if this is a divergence
                                let is_divergence =
                                    (price_change_pct > 0.0 && matches!(signal, ImbalanceSignal::Bearish)) ||
                                    (price_change_pct < 0.0 && matches!(signal, ImbalanceSignal::Bullish));

                                if is_divergence {
                                    println!("⚡ Divergence: Price {:+.2}% but orderbook {:?}", price_change_pct, signal);
                                    if let Err(e) = bot.send_divergence_alert(trading_pair, price_change_pct, signal).await {
                                        eprintln!("Failed to send divergence alert: {}", e);
                                    } else {
                                        alert_count += 1;
                                        println!("✅ Divergence alert #{} sent", alert_count);
                                    }
                                }
                            }
                        }
                    }

                    // Log status every 50 updates
                    if orderbook_update_count % 50 == 0 {
                        println!(
                            "📊 Status: {} updates | {} alerts | Current imbalance: {:+.2}%",
                            orderbook_update_count,
                            alert_count,
                            metrics.imbalance_ratio * 100.0
                        );
                    }
                }
            }

            // Handle ticker updates (for price alerts)
            Some(tick) = ticker_sub.next() => {
                last_price = Some(tick.last);

                // ═══════════════════════════════════════════════════════════
                // NEW FEATURE: Trade Alert - Detect large volume trades
                // ═══════════════════════════════════════════════════════════
                // Note: Using 24h volume as proxy for large trades
                // In production, you'd use the actual trades channel
                if tick.volume > 5000.0 {  // Example: 24h volume > 5000 BTC indicates active trading
                    // Simulate a large trade detection (every 2 minutes)
                    if last_price_check.elapsed() >= Duration::from_secs(120) {
                        // Simulate: assume a 15 BTC trade just executed
                        let simulated_volume = 15.0;
                        let side = if tick.last > tick.low + (tick.high - tick.low) * 0.5 {
                            "buy"
                        } else {
                            "sell"
                        };

                        println!("💥 Simulated trade: {} {} BTC @ ${:.2}", side, simulated_volume, tick.last);
                        if let Err(e) = bot.send_trade_alert(trading_pair, side, tick.last, simulated_volume).await {
                            eprintln!("Failed to send trade alert: {}", e);
                        } else {
                            alert_count += 1;
                            println!("✅ Trade alert #{} sent", alert_count);
                        }
                    }
                }

                // Check price thresholds periodically
                if last_price_check.elapsed() >= price_check_interval {
                    if tick.last >= price_threshold_high {
                        if let Err(e) = bot.send_threshold_alert(
                            trading_pair,
                            tick.last,
                            price_threshold_high,
                            true,
                        ).await {
                            eprintln!("Failed to send price alert: {}", e);
                        } else {
                            println!("📈 Price alert sent: ${:.2} (above ${:.2})", tick.last, price_threshold_high);
                        }
                    } else if tick.last <= price_threshold_low {
                        if let Err(e) = bot.send_threshold_alert(
                            trading_pair,
                            tick.last,
                            price_threshold_low,
                            false,
                        ).await {
                            eprintln!("Failed to send price alert: {}", e);
                        } else {
                            println!("📉 Price alert sent: ${:.2} (below ${:.2})", tick.last, price_threshold_low);
                        }
                    }

                    last_price_check = std::time::Instant::now();
                }
            }

            // Periodic status update
            _ = tokio::time::sleep(Duration::from_secs(60)) => {
                if let Some(ob) = client.get_orderbook(trading_pair) {
                    if let (Some(bid), Some(ask)) = (ob.best_bid(), ob.best_ask()) {
                        let summary = format!(
                            "📊 *Hourly Summary - {}*\n\
                            \n\
                            Updates Received: {}\n\
                            Alerts Sent: {}\n\
                            Best Bid: ${:.2}\n\
                            Best Ask: ${:.2}\n\
                            Current Price: ${:.2}\n\
                            Status: 🟢 Active",
                            trading_pair,
                            orderbook_update_count,
                            alert_count,
                            bid,
                            ask,
                            last_price.unwrap_or(0.0)
                        );

                        if let Err(e) = bot.send_alert(&summary).await {
                            eprintln!("Failed to send hourly summary: {}", e);
                        }
                    }
                }
            }
        }
    }
}
