---
name: Bug Report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

## 🐛 Bug Description

A clear and concise description of what the bug is.

## 🔄 Steps to Reproduce

1. Go to '...'
2. Run command '....'
3. Call function '....'
4. See error

## ✅ Expected Behavior

A clear and concise description of what you expected to happen.

## ❌ Actual Behavior

A clear and concise description of what actually happened.

## 💻 Code Example

```rust
// Minimal code example that reproduces the issue
use kraky::KrakyClient;

#[tokio::main]
async fn main() {
    let client = KrakyClient::connect().await.unwrap();
    // ...
}
```

## 📋 Environment

- **OS**: [e.g., macOS 14.0, Ubuntu 22.04, Windows 11]
- **Rust Version**: [e.g., 1.75.0] (run `rustc --version`)
- **Kraky Version**: [e.g., 0.1.0] (from Cargo.toml)
- **Features Enabled**: [e.g., `full`, `trades,ticker`, etc.]

## 📝 Error Messages / Logs

```
Paste any error messages or relevant log output here
```

## 📸 Screenshots

If applicable, add screenshots to help explain your problem.

## 🔍 Additional Context

Add any other context about the problem here.

## ✅ Checklist

- [ ] I have searched existing issues to avoid duplicates
- [ ] I have provided a minimal code example
- [ ] I have included error messages/logs
- [ ] I have specified my environment details
