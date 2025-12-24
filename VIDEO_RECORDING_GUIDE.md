# Demo Video Recording Guide

## 🎬 Quick Reference (3-5 Minutes Total)

**Recommended Tools:**
- **macOS:** QuickTime Player (built-in screen recording)
- **Windows:** OBS Studio (free) or Xbox Game Bar (built-in)
- **Linux:** OBS Studio or SimpleScreenRecorder
- **Cross-platform:** Loom (free tier available)

---

## 🎯 Video Structure (Recommended: 3-4 minutes)

### Opening (15 seconds)
**What to show:** Terminal with project directory
**What to say:**
> "Hi! I'm presenting Kraky - a production-ready Rust SDK for the Kraken Exchange with unique orderbook imbalance detection and WebSocket trading. Let me show you what makes it special."

**Commands:**
```bash
cd kraky
ls -la
```

---

### Demo 1: Live Orderbook (30 seconds)
**What to say:**
> "First, let's see live orderbook data from Kraken's BTC/USD market."

**Command:**
```bash
cargo run --example orderbook
```

**While running, highlight:**
- "See the real-time bid/ask spread updating"
- "The SDK automatically manages orderbook state"
- "Updates are coming in 10+ times per second"

**Action:** Let it run for 15-20 seconds, then Ctrl+C

---

### Demo 2: Imbalance Detection (30 seconds)
**What to say:**
> "Now here's our unique feature - orderbook imbalance detection generating trading signals in real-time."

**Command:**
```bash
cargo run --example liquidity_monitor --features analytics
```

**While running, highlight:**
- "Watch the bullish and bearish signals"
- "The green/red indicators show buy/sell pressure"
- "This is based on real-time bid/ask volume analysis"

**Action:** Wait until you see a signal change, then Ctrl+C

---

### Demo 3: CSV Export (45 seconds)
**What to say:**
> "Kraky can export all this data to CSV files for backtesting and analysis. Let's run a 30-second export."

**Command:**
```bash
cargo run --example export_to_csv --features trades,analytics
```

**While running:**
> "It's streaming orderbook snapshots and individual trades simultaneously."

**After it completes:**
```bash
ls -lh *.csv
echo "Let's see what the data looks like:"
head -5 orderbook_BTCUSD_*.csv
```

**What to say:**
> "Perfect for Excel, Python, R, or any analysis tool."

---

### Demo 4: All Features (30 seconds)
**What to say:**
> "Let me show you our comprehensive demo that showcases all 9 major features at once."

**Command:**
```bash
cargo run --example demo --features full
```

**While running, point out:**
- "Feature 1: WebSocket connection ✅"
- "Feature 2: Connection events ✅"
- "Feature 3: State monitoring ✅"
- "Feature 4: Multiple subscriptions ✅"
- (Let a few more appear)

**Action:** Let it run for 20 seconds showing live updates, then Ctrl+C

---

### Demo 5: Tests (15 seconds)
**What to say:**
> "The SDK is production-ready with comprehensive testing. Let's verify."

**Command:**
```bash
cargo test
```

**What to say while tests run:**
> "You'll see all 25 tests pass - covering orderbook operations, subscription handling, error parsing, and reconnection logic."

---

### Demo 6: Modular Features (20 seconds)
**What to say:**
> "One last thing - Kraky is completely modular. You only compile what you need."

**Commands:**
```bash
cat Cargo.toml | grep -A 15 "\[features\]"
```

**What to say:**
> "Each feature is optional - trading, analytics, Telegram alerts. The core SDK is just 7.2 MB, even with full trading support it's only 8.5 MB."

---

### Closing (15 seconds)
**What to say:**
> "So to recap: Kraky is the only Kraken SDK with orderbook imbalance detection and WebSocket trading. It's fast, modular, production-ready with 25 tests, and has 16 working examples. All code is open-source under MIT license. Thanks for watching!"

**What to show:** GitHub repo page or README

---

## 📝 Recording Tips

### Before Recording
1. **Clean your terminal**
   ```bash
   clear
   cd ~/kraken-sdk  # Or wherever your project is
   ```

2. **Increase terminal font size**
   - Make it readable for viewers
   - Zoom in terminal: Cmd+Plus (Mac) or Ctrl+Plus (Windows/Linux)
   - Recommended: 16-18pt font

3. **Close unnecessary applications**
   - Hide distracting desktop icons
   - Close notifications
   - Close other terminal tabs/windows

4. **Test your microphone**
   - Do a 10-second test recording
   - Verify audio is clear

5. **Prepare commands in a text file**
   - Have all commands ready to copy-paste
   - Reduces typing errors during recording

### During Recording
- **Speak clearly and at moderate pace**
- **Don't worry about being perfect** - authentic is better than scripted
- **Let demos run** - show actual live data
- **Point out unique features** - imbalance detection, WebSocket trading
- **Show enthusiasm** - you built something cool!

### After Recording
1. **Watch the video** - make sure audio and video are clear
2. **Check timing** - aim for 3-5 minutes (judges are busy)
3. **Upload to YouTube**
   - Set visibility to "Unlisted" (not private, not public)
   - Add title: "Kraky - Rust SDK for Kraken Exchange | Kraken Forge Hackathon"
   - Add description linking to GitHub repo

---

## 🎬 Alternative: Shorter Version (2 minutes)

If you're short on time, here's a condensed version:

### Quick Demo Script (2 min)
1. **Intro** (10s): "Kraky - Rust SDK with unique imbalance detection and WebSocket trading"
2. **Live orderbook** (20s): `cargo run --example orderbook`
3. **Imbalance signals** (20s): `cargo run --example liquidity_monitor --features analytics`
4. **Full demo** (30s): `cargo run --example demo --features full`
5. **Tests** (20s): `cargo test`
6. **Closing** (20s): "Production-ready, 25 tests, 16 examples. Check out the repo!"

---

## 🎥 Recording Commands (All in One Place)

Copy these for easy access during recording:

```bash
# Setup
clear
cd kraken-sdk

# Demo 1
cargo run --example orderbook
# Ctrl+C after 15 seconds

# Demo 2
cargo run --example liquidity_monitor --features analytics
# Ctrl+C after 20 seconds

# Demo 3
cargo run --example export_to_csv --features trades,analytics
# Wait for completion (30 seconds)
ls -lh *.csv
head -5 orderbook_BTCUSD_*.csv

# Demo 4
cargo run --example demo --features full
# Ctrl+C after 20 seconds

# Demo 5
cargo test

# Demo 6
cat Cargo.toml | grep -A 15 "\[features\]"

# Show examples count
ls -1 examples/ | wc -l
```

---

## 📤 Upload Checklist

- [ ] Video is 2-5 minutes long
- [ ] Audio is clear
- [ ] Terminal text is readable
- [ ] All demos work as shown
- [ ] Video uploaded to YouTube/Vimeo
- [ ] Visibility set to **Unlisted** (important!)
- [ ] Video URL added to SUBMISSION.md
- [ ] Video URL added to TAIKAI submission

---

## 🚨 Troubleshooting

**Problem:** Demo hangs or doesn't connect
**Solution:** Check internet connection. If Kraken WebSocket is down, use pre-recorded output or show code instead.

**Problem:** Terminal text too small
**Solution:** Increase font size: Cmd/Ctrl + Plus

**Problem:** Too much background noise
**Solution:** Record in quiet room or use noise-canceling software

**Problem:** Made a mistake while recording
**Solution:** Keep going! Authentic is better than perfect. Or start over if it's major.

**Problem:** Video file too large
**Solution:** Compress with HandBrake or upload directly to YouTube (they handle compression)

---

## ✨ Pro Tips

1. **Practice once** before recording - run through all demos to ensure they work
2. **Smile!** Even in audio-only, enthusiasm comes through
3. **Keep it moving** - judges are watching many videos, respect their time
4. **Highlight unique features** - imbalance detection and WebSocket trading are your differentiators
5. **Show real data** - live demos are more impressive than static screenshots

---

**You've got this! The hard work is done - now just show it off!** 🎬🚀
