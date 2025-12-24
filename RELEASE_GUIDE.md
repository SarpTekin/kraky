# 🚀 Kraky Release Guide

This guide walks you through publishing Kraky to both GitHub (for collaboration) and crates.io (for Rust ecosystem).

---

## 📋 Quick Checklist

### Ready to Publish?

- [x] All tests passing (`cargo test`)
- [x] Code formatted (`cargo fmt`)
- [x] No clippy warnings (`cargo clippy`)
- [x] Documentation complete
- [x] Examples working
- [x] LICENSE file present
- [x] CHANGELOG.md created
- [x] CONTRIBUTING.md created
- [x] GitHub templates created
- [x] Cargo.toml metadata complete
- [ ] **GitHub pushed** (follow GitHub section below)
- [ ] **Published to crates.io** (follow Publishing section below)

---

## 🐙 Part 1: GitHub Release

### Step 1: Final Verification

```bash
# Ensure all changes are committed
git status

# Run final tests
cargo test --all-features

# Verify examples compile
cargo build --examples --all-features
```

### Step 2: Push to GitHub

```bash
# Push all commits
git push origin main

# Verify on GitHub
# Go to: https://github.com/sarptekin/kraky
# Check that all files are there
```

### Step 3: Create GitHub Release

1. **Go to Releases**: https://github.com/sarptekin/kraky/releases
2. **Click**: "Draft a new release"
3. **Create tag**: `v0.1.0`
4. **Release title**: `Kraky v0.1.0 - Initial Release`
5. **Description**: Copy from CHANGELOG.md (the 0.1.0 section)
6. **Publish release**

### Step 4: Verify GitHub

- [ ] Repository is public
- [ ] README displays correctly
- [ ] All examples are visible
- [ ] CONTRIBUTING.md is accessible
- [ ] LICENSE is visible
- [ ] GitHub templates work (try creating an issue)

---

## 📦 Part 2: Publish to crates.io

### Step 1: Pre-Publish Checks

```bash
# 1. Format and lint
cargo fmt --all
cargo clippy --all-features -- -D warnings

# 2. Run all tests
cargo test --all-features

# 3. Check documentation builds
cargo doc --no-deps --all-features

# 4. List what will be published
cargo package --list

# 5. Build the package
cargo package

# 6. Verify package size
ls -lh target/package/kraky-0.1.0.crate
```

### Step 2: Login to crates.io

**First time only:**

```bash
# 1. Go to https://crates.io
# 2. Sign in with GitHub
# 3. Go to https://crates.io/me
# 4. Click "New Token"
# 5. Run:
cargo login
# 6. Paste your token when prompted
```

### Step 3: Verify Package Name Available

```bash
# Check if "kraky" is available:
# Visit: https://crates.io/crates/kraky

# If taken, you need to:
# - Choose different name in Cargo.toml
# - Update all documentation
```

### Step 4: Dry Run

```bash
# Test publishing WITHOUT actually publishing
cargo publish --dry-run

# This will:
# - Build the package
# - Verify it
# - Check dependencies
# - Simulate upload
```

### Step 5: Publish!

```bash
# IMPORTANT: This CANNOT be undone!
cargo publish

# You should see:
# Uploading kraky v0.1.0 (/path/to/kraky)
#     Updating crates.io index
```

### Step 6: Verify on crates.io

Wait 2-5 minutes, then check:

- **Crate page**: https://crates.io/crates/kraky
- **Documentation**: https://docs.rs/kraky (may take 10-30 min to build)
- **Versions**: Verify 0.1.0 appears

### Step 7: Add Badges to README

Add these at the top of README.md:

```markdown
[![Crates.io](https://img.shields.io/crates/v/kraky.svg)](https://crates.io/crates/kraky)
[![Documentation](https://docs.rs/kraky/badge.svg)](https://docs.rs/kraky)
[![License](https://img.shields.io/crates/l/kraky.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/kraky.svg)](https://crates.io/crates/kraky)
[![Build Status](https://github.com/sarptekin/kraky/workflows/CI/badge.svg)](https://github.com/sarptekin/kraky/actions)
```

Then commit and push:

```bash
git add README.md
git commit -m "docs: add crates.io badges"
git push origin main
```

---

## 🎉 Post-Release

### Announce Your Release

#### On Crates.io
Your crate is now discoverable at: https://crates.io/crates/kraky

#### On GitHub
- Update repository description
- Add topics: `rust`, `kraken`, `websocket`, `trading`, `crypto`

#### Social Media (Optional)
- Tweet about it
- Post in Rust Discord/Reddit
- Share in relevant communities

### Monitor Your Project

- **GitHub Issues**: Respond to bug reports and feature requests
- **crates.io Stats**: Track downloads and usage
- **Documentation**: Monitor docs.rs build status

---

## 🔄 Future Releases

### For Bug Fixes (0.1.0 → 0.1.1)

```bash
# 1. Make your fixes
# 2. Update version in Cargo.toml
version = "0.1.1"

# 3. Update CHANGELOG.md
## [0.1.1] - 2024-XX-XX
### Fixed
- Fixed bug in orderbook parsing

# 4. Commit and publish
git add Cargo.toml CHANGELOG.md src/
git commit -m "fix: correct orderbook parsing issue"
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin main --tags
cargo publish
```

### For New Features (0.1.0 → 0.2.0)

```bash
# Same as above but:
version = "0.2.0"

## [0.2.0] - 2024-XX-XX
### Added
- New feature description
```

### For Breaking Changes (0.1.0 → 1.0.0)

```bash
# Same as above but:
version = "1.0.0"

## [1.0.0] - 2024-XX-XX
### Breaking Changes
- Description of breaking changes
### Migration Guide
- How users should update their code
```

---

## ⚠️ Important Notes

### You CANNOT:
- Delete a published version
- Modify a published version
- Reuse a version number
- Undo a publish

### You CAN:
- Yank a broken version (prevents new installs): `cargo yank --vers 0.1.0`
- Publish a new version with fixes
- Update documentation on docs.rs

---

## 🛠️ Troubleshooting

### "crate name is already taken"
- Check: https://crates.io/crates/kraky
- If taken, choose different name in Cargo.toml

### "failed to verify package tarball"
```bash
# Test build manually:
cargo package
cd target/package/kraky-0.1.0
cargo build --all-features
```

### "documentation failed to build"
```bash
# Test docs locally:
cargo doc --no-deps --all-features
# Fix any errors, then publish
```

### "some files are missing"
```bash
# Check what will be included:
cargo package --list
# Add missing files to git or include in Cargo.toml
```

---

## 📚 Resources

- [PUBLISHING.md](PUBLISHING.md) - Detailed publishing guide
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contributor guidelines
- [CHANGELOG.md](CHANGELOG.md) - Version history
- [Cargo Book](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)

---

## ✅ Final Checklist

Before you start:

- [ ] All code committed to git
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Examples working
- [ ] CHANGELOG.md updated
- [ ] Cargo.toml version correct
- [ ] GitHub repository pushed
- [ ] crates.io account created
- [ ] cargo login completed

Ready to publish:

- [ ] `cargo package --list` checked
- [ ] `cargo publish --dry-run` successful
- [ ] Name available on crates.io
- [ ] Backup created (optional but recommended)

After publishing:

- [ ] Verified on https://crates.io/crates/kraky
- [ ] Documentation building on https://docs.rs/kraky
- [ ] GitHub release created
- [ ] Badges added to README
- [ ] Social announcements (optional)

---

**Congratulations! You're ready to release Kraky to the world!** 🎉🚀

For questions, check the detailed guides or open an issue on GitHub.
