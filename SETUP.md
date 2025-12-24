# Kraky SDK - Setup Guide

This guide will help you set up and run Kraky SDK examples, including those that require API credentials.

---

## 📋 Quick Start (No Credentials Needed)

Most examples work immediately without any setup:

```bash
# Clone the repository
git clone https://github.com/SarpTekin/kraky.git
cd kraky

# Run basic examples (no credentials needed)
cargo run --example orderbook                              # Live BTC/USD orderbook
cargo run --example trades --features trades               # Live trades
cargo run --example ticker --features ticker               # Live ticker
cargo run --example demo --features full                   # Full feature demo
cargo run --example multi_pair_monitor --features analytics # Multi-pair monitor
cargo run --example export_to_csv --features trades,analytics # Export to CSV

# Run tests
cargo test
```

**These examples work WITHOUT any credentials or configuration!**

---

## 🔐 Setting Up Credentials (Optional)

Some advanced examples require API credentials from external services. Only set these up if you want to use the specific features.

### Option 1: Environment Variables (Recommended)

The simplest way is to set environment variables in your shell:

```bash
# For Telegram features
export TELEGRAM_BOT_TOKEN="your_bot_token_here"
export TELEGRAM_CHAT_ID="your_chat_id_here"

# For Kraken private/trading features
export KRAKEN_API_KEY="your_api_key_here"
export KRAKEN_API_SECRET="your_base64_secret_here"
```

**Tip:** Add these to your `~/.bashrc`, `~/.zshrc`, or `~/.profile` to make them permanent.

### Option 2: .env File (Recommended for Development)

1. **Copy the template:**
   ```bash
   cp .env.example .env
   ```

2. **Edit the `.env` file:**
   ```bash
   nano .env  # or use your favorite editor
   ```

3. **Fill in your credentials** (see sections below for how to get them)

4. **Load environment variables** before running examples:
   ```bash
   # On Unix/Linux/macOS
   export $(cat .env | xargs)

   # Or use a tool like direnv or dotenv
   ```

**Security Note:** The `.env` file is automatically ignored by git (listed in `.gitignore`), so your credentials won't be accidentally committed.

---

## 🤖 Getting Telegram Credentials

Required for examples:
- `simple_price_alerts`
- `whale_watcher`
- `telegram_imbalance_bot`
- `telegram_private_alerts`
- `telegram_trading_bot`
- `telegram_trading_demo`

### Step 1: Create a Telegram Bot

1. **Open Telegram** and search for `@BotFather`
2. **Send** `/newbot`
3. **Follow the prompts:**
   - Choose a name for your bot (e.g., "Kraky Alerts")
   - Choose a username (must end in 'bot', e.g., "kraky_alerts_bot")
4. **Save the token** - BotFather will give you a token like:
   ```
   123456789:ABCdefGHIjklMNOpqrsTUVwxyz
   ```
   This is your `TELEGRAM_BOT_TOKEN`

### Step 2: Get Your Chat ID

1. **Open Telegram** and search for `@userinfobot`
2. **Send** any message to the bot
3. **Copy the ID** it sends back (e.g., `987654321`)
   This is your `TELEGRAM_CHAT_ID`

### Step 3: Set Environment Variables

```bash
export TELEGRAM_BOT_TOKEN="123456789:ABCdefGHIjklMNOpqrsTUVwxyz"
export TELEGRAM_CHAT_ID="987654321"
```

### Step 4: Test It

```bash
cargo run --example simple_price_alerts --features telegram-alerts
```

You should receive a message on Telegram!

---

## 🔑 Getting Kraken API Credentials

Required for examples:
- `auth_example`
- `telegram_trading_bot`
- `telegram_private_alerts`

### Step 1: Create API Key on Kraken

1. **Log in** to [kraken.com](https://www.kraken.com)
2. **Navigate** to Settings → API
3. **Click** "Generate New Key"
4. **Set permissions** based on what you need:

   **For market data only:**
   - No special permissions needed

   **For account information:**
   - ✅ Query Funds
   - ✅ Query Open Orders & Trades
   - ✅ Query Closed Orders & Trades

   **For trading (be careful!):**
   - ✅ Create & Modify Orders
   - ✅ Cancel/Close Orders
   - ⚠️ **Only enable if you understand the risks**

5. **Set description:** e.g., "Kraky SDK - Development"
6. **Leave nonce window** at default (unless you know what you're doing)
7. **Click** "Generate Key"

### Step 2: Save Your Credentials

You'll see two values:

1. **API Key** - A long string like: `abc123XYZ...`
2. **Private Key** - A Base64 string (only shown once!) like: `dGVzdC9zZWNyZXQ=...`

⚠️ **IMPORTANT:** The Private Key is only shown once! Save it immediately.

### Step 3: Set Environment Variables

```bash
export KRAKEN_API_KEY="your_api_key_here"
export KRAKEN_API_SECRET="your_base64_private_key_here"
```

### Step 4: Test It

```bash
cargo run --example auth_example --features private
```

---

## 🛡️ Security Best Practices

### Do's ✅

- ✅ Use environment variables or `.env` files for credentials
- ✅ Keep your `.env` file local (it's in `.gitignore`)
- ✅ Use API keys with minimal required permissions
- ✅ Create separate API keys for testing vs production
- ✅ Rotate API keys regularly
- ✅ Delete API keys when no longer needed
- ✅ Use validation mode for testing trading bots

### Don'ts ❌

- ❌ Never hardcode credentials in source code
- ❌ Never commit `.env` files to git
- ❌ Never share your API credentials
- ❌ Never give trading permissions unless necessary
- ❌ Never use production API keys for testing
- ❌ Never push code containing credentials to GitHub

### If You Accidentally Commit Credentials

1. **Immediately revoke** the API key on Kraken/Telegram
2. **Generate new** credentials
3. **Update your local** `.env` file
4. Consider using `git filter-branch` or BFG Repo-Cleaner to remove from history

---

## 📂 Project Structure

```
kraky/
├── .env.example          # Template for environment variables
├── .env                  # Your actual credentials (git-ignored)
├── .gitignore            # Ensures .env is never committed
├── examples/
│   ├── orderbook.rs      # ✅ No credentials needed
│   ├── trades.rs         # ✅ No credentials needed
│   ├── ticker.rs         # ✅ No credentials needed
│   ├── demo.rs           # ✅ No credentials needed
│   ├── simple_price_alerts.rs      # 🔐 Needs Telegram
│   ├── telegram_trading_bot.rs     # 🔐 Needs Telegram + Kraken
│   └── ...
└── ...
```

---

## 🧪 Verifying Your Setup

### 1. Check Environment Variables

```bash
# Check if variables are set
echo $TELEGRAM_BOT_TOKEN
echo $TELEGRAM_CHAT_ID
echo $KRAKEN_API_KEY
echo $KRAKEN_API_SECRET

# Should show your values (or be empty if not set)
```

### 2. Run Examples Without Credentials

```bash
cargo run --example orderbook
# Should work immediately and show live BTC/USD data
```

### 3. Run Examples With Telegram

```bash
cargo run --example simple_price_alerts --features telegram-alerts
# Should send you a Telegram message
```

### 4. Run Tests

```bash
cargo test
# Should show: test result: ok. 25 passed; 0 failed
```

---

## 🐛 Troubleshooting

### "Please set TELEGRAM_BOT_TOKEN environment variable"

**Problem:** Environment variable not set

**Solution:**
```bash
export TELEGRAM_BOT_TOKEN="your_token_here"
export TELEGRAM_CHAT_ID="your_chat_id_here"
```

### "Invalid TELEGRAM_CHAT_ID"

**Problem:** Chat ID must be a number

**Solution:** Make sure your chat ID is just numbers (e.g., `987654321`), not a username

### "Failed to send Telegram message"

**Problem:** Either the bot token is wrong or you haven't started a chat with the bot

**Solution:**
1. Find your bot on Telegram (search for the username you created)
2. Send it a message first (e.g., `/start`)
3. Then run the example again

### ".env file not being loaded"

**Problem:** Environment variables in `.env` aren't automatically loaded

**Solution:**
```bash
# Load manually before running examples:
export $(cat .env | xargs)

# OR install and use direnv or dotenv CLI tools
```

### "Kraken API error"

**Problem:** API key permissions or format issue

**Solution:**
1. Verify the API key and secret are correct (copy-paste from Kraken)
2. Check the API key has the required permissions
3. Ensure the secret is the Base64-encoded version

---

## 📚 Examples by Credential Requirement

### No Credentials Required ✅
```bash
cargo run --example orderbook
cargo run --example trades --features trades
cargo run --example ticker --features ticker
cargo run --example ohlc --features ohlc
cargo run --example multi_subscribe --features full
cargo run --example demo --features full
cargo run --example benchmark
cargo run --example multi_pair_monitor --features analytics
cargo run --example export_to_csv --features trades,analytics
cargo run --example export_multi_csv --features trades,analytics
cargo run --example liquidity_monitor --features analytics  # Works without, better with Telegram
```

### Telegram Required 🤖
```bash
cargo run --example simple_price_alerts --features telegram-alerts
cargo run --example whale_watcher --features telegram-alerts
cargo run --example telegram_imbalance_bot --features telegram-alerts,analytics
cargo run --example telegram_private_alerts --features telegram-alerts,private
cargo run --example telegram_trading_demo --features telegram,trading
```

### Telegram + Kraken Required 🔐
```bash
cargo run --example telegram_trading_bot --features telegram,trading,private
```

### Kraken Only 🔑
```bash
cargo run --example auth_example --features private  # Works without (demo mode)
```

---

## 🚀 Ready to Go!

Now you're all set up! Start with the examples that don't require credentials, then configure Telegram and/or Kraken credentials as needed.

**Recommended learning path:**

1. **Start simple:** `cargo run --example orderbook`
2. **Try analytics:** `cargo run --example liquidity_monitor --features analytics`
3. **Export data:** `cargo run --example export_to_csv --features trades,analytics`
4. **Add Telegram:** Set up bot and try `simple_price_alerts`
5. **Advanced:** Try the full `demo` or build your own bot

**Questions?** Check the [README.md](README.md) or open an issue on GitHub!

Happy trading! 🐙🚀
