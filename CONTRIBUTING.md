# Contributing to Kraky

Thank you for your interest in contributing to Kraky! This document provides guidelines and instructions for contributing.

## 🎯 Ways to Contribute

- **Bug Reports**: Found a bug? Open an issue with details
- **Feature Requests**: Have an idea? We'd love to hear it
- **Code Contributions**: Submit pull requests for fixes or features
- **Documentation**: Improve docs, examples, or guides
- **Testing**: Help test new features or report issues

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- (Optional) Kraken API credentials for testing private features
- (Optional) Telegram bot token for testing alert features

### Setup

```bash
# Clone the repository
git clone https://github.com/sarptekin/kraky.git
cd kraky

# Run tests
cargo test

# Run examples
cargo run --example orderbook
cargo run --example demo --features full

# Check code quality
cargo clippy
cargo fmt --check
```

---

## 📝 Development Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/kraky.git
cd kraky
git remote add upstream https://github.com/sarptekin/kraky.git
```

### 2. Create a Branch

```bash
# Create a descriptive branch name
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write clear, concise code
- Follow Rust conventions and idioms
- Add tests for new functionality
- Update documentation as needed
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Test specific feature combinations
cargo test --features trades,ticker,analytics
cargo test --features full

# Run examples to verify functionality
cargo run --example orderbook
cargo run --example demo --features full

# Check documentation builds
cargo doc --no-deps --features full
```

### 5. Commit

Follow conventional commit format:

```bash
# Format: type(scope): description
git commit -m "feat(client): add support for new Kraken API endpoint"
git commit -m "fix(orderbook): correct imbalance calculation"
git commit -m "docs(readme): update installation instructions"
git commit -m "test(subscriptions): add tests for reconnection"
```

**Commit types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

### 6. Push and Create PR

```bash
git push origin your-branch-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Reference to related issues (e.g., "Fixes #123")
- Screenshots/examples if applicable

---

## 🧪 Testing Guidelines

### Writing Tests

- Place unit tests in the same file as the code (using `#[cfg(test)]`)
- Place integration tests in `tests/` directory
- Test edge cases and error conditions
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_spread_calculation() {
        // Test implementation
    }

    #[test]
    fn test_imbalance_with_empty_orderbook() {
        // Edge case test
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_orderbook_spread

# With output
cargo test -- --nocapture

# With specific features
cargo test --features analytics
```

---

## 📚 Documentation Guidelines

### Code Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Explain parameters and return values
- Document error conditions

```rust
/// Calculates the orderbook imbalance ratio
///
/// Returns a value between -1.0 (completely bearish) and +1.0 (completely bullish).
/// A value of 0.0 indicates balanced bid/ask volume.
///
/// # Examples
///
/// ```
/// use kraky::Orderbook;
///
/// let ob = Orderbook::new();
/// let imbalance = ob.imbalance();
/// assert!(imbalance >= -1.0 && imbalance <= 1.0);
/// ```
///
/// # Returns
///
/// * `f64` - Imbalance ratio from -1.0 to +1.0
pub fn imbalance(&self) -> f64 {
    // Implementation
}
```

### Example Programs

- Keep examples focused on one feature
- Add clear comments explaining each step
- Include error handling
- Show realistic usage

---

## 🎨 Code Style

### Formatting

```bash
# Auto-format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --features full

# Fix auto-fixable issues
cargo clippy --fix
```

### Conventions

- Use descriptive variable names
- Keep functions focused and small
- Prefer iterators over loops where appropriate
- Use `?` operator for error propagation
- Add `#[must_use]` where applicable
- Use `#[inline]` judiciously for hot paths

---

## 🔒 Security

### Reporting Security Issues

**DO NOT** open public issues for security vulnerabilities.

Instead:
1. Email security concerns to: [your-email]
2. Include "SECURITY" in the subject line
3. Provide detailed description and steps to reproduce

### Security Best Practices

- Never commit credentials or API keys
- Use environment variables for sensitive data
- Validate all input from external sources
- Follow Rust's safety guidelines
- Use `cargo audit` to check for vulnerabilities

---

## 📦 Feature Flags

When adding new features, consider:

- Should it be optional? (Use feature flags)
- What dependencies does it require?
- Does it increase binary size significantly?
- Is it useful to most users?

### Adding a New Feature Flag

1. Add to `Cargo.toml`:
```toml
[features]
your-feature = ["dep:some-crate"]

[dependencies]
some-crate = { version = "1.0", optional = true }
```

2. Gate code with `#[cfg(feature = "your-feature")]`

3. Add example requiring the feature:
```toml
[[example]]
name = "your_example"
path = "examples/your_example.rs"
required-features = ["your-feature"]
```

4. Document in README and example comments

---

## 🐛 Bug Reports

### Good Bug Report Checklist

- [ ] Descriptive title
- [ ] Steps to reproduce
- [ ] Expected behavior
- [ ] Actual behavior
- [ ] Environment (OS, Rust version, etc.)
- [ ] Minimal code example
- [ ] Error messages/logs

### Example Bug Report

```markdown
**Description**
Orderbook imbalance calculation returns NaN when orderbook is empty

**To Reproduce**
1. Connect to Kraken
2. Subscribe to orderbook
3. Call `imbalance()` before receiving any data
4. Returns `NaN` instead of `0.0`

**Expected Behavior**
Should return `0.0` or `None` for empty orderbook

**Environment**
- OS: macOS 14.0
- Rust: 1.75.0
- Kraky: 0.1.0

**Code Example**
\`\`\`rust
let client = KrakyClient::connect().await?;
let mut sub = client.subscribe_orderbook("BTC/USD", 10).await?;
let ob = client.get_orderbook("BTC/USD").unwrap();
println!("{}", ob.imbalance()); // Prints: NaN
\`\`\`
```

---

## 💡 Feature Requests

### Good Feature Request Checklist

- [ ] Clear use case
- [ ] Proposed API design
- [ ] Alternatives considered
- [ ] Implementation suggestions (optional)

### Example Feature Request

```markdown
**Feature Request: Add support for OHLC subscriptions**

**Use Case**
I want to build a candlestick chart for BTC/USD using 1-minute intervals without fetching from REST API.

**Proposed API**
\`\`\`rust
let ohlc_sub = client.subscribe_ohlc("BTC/USD", Interval::Min1).await?;
while let Some(candle) = ohlc_sub.next().await {
    println!("Open: {}, Close: {}", candle.open, candle.close);
}
\`\`\`

**Alternatives**
- Use REST API (slower, rate-limited)
- Calculate OHLC from trade stream (complex)

**Additional Context**
Kraken WebSocket API supports OHLC subscriptions: https://docs.kraken.com/websockets-v2/
```

---

## 📋 Pull Request Guidelines

### Before Submitting

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Examples added/updated if needed
- [ ] CHANGELOG.md updated (for significant changes)

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature causing existing functionality to change)
- [ ] Documentation update

## Testing
- Describe tests added
- Manual testing performed

## Checklist
- [ ] Code follows project style
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests added/updated
- [ ] All tests pass
```

---

## 🎓 Resources

### Learning Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Project-Specific

- [Kraken WebSocket API v2 Docs](https://docs.kraken.com/websockets-v2/)
- [Project README](README.md)
- [Architecture Guide](ARCHITECTURE.md) (if exists)

### Tools

- [cargo-edit](https://github.com/killercup/cargo-edit) - Manage dependencies
- [cargo-watch](https://github.com/watchexec/cargo-watch) - Auto-rebuild on changes
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit) - Security audits

---

## 📧 Contact

- **Issues**: [GitHub Issues](https://github.com/sarptekin/kraky/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sarptekin/kraky/discussions)
- **Email**: [your-email] (for security issues only)

---

## 📜 License

By contributing to Kraky, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to Kraky!** 🐙🚀
