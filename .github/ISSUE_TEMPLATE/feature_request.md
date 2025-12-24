---
name: Feature Request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

## 💡 Feature Description

A clear and concise description of the feature you'd like to see.

## 🎯 Use Case

Describe the problem this feature would solve or the use case it would enable.

**Example:**
> I want to build a trading dashboard that shows real-time P&L calculations, but the SDK doesn't provide a way to track position changes.

## 📝 Proposed API Design

Show how you envision the feature would be used:

```rust
// Example of proposed API
use kraky::KrakyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakyClient::connect().await?;

    // Your proposed feature usage
    let new_feature = client.some_new_method().await?;

    Ok(())
}
```

## 🔄 Alternatives Considered

Describe any alternative solutions or features you've considered.

**Example:**
- Use REST API instead (slower, rate-limited)
- Calculate manually from trade stream (complex)
- Use a third-party library (adds dependency)

## 📊 Impact

**Who would benefit from this feature?**
- [ ] All users
- [ ] Algorithmic traders
- [ ] Casual developers
- [ ] Enterprise users
- [ ] Other: ___________

**Would this be a breaking change?**
- [ ] Yes
- [ ] No
- [ ] Unsure

## 🔧 Implementation Suggestions (Optional)

If you have ideas about how this could be implemented, share them here.

## 📎 Additional Context

Add any other context, screenshots, or examples about the feature request.

## 🔗 Related Issues

Link any related issues or discussions here.

## ✅ Checklist

- [ ] I have searched existing issues to avoid duplicates
- [ ] I have provided a clear use case
- [ ] I have shown proposed API design
- [ ] I have considered alternatives
