# Publishing Kraky to crates.io

This guide explains how to publish Kraky to crates.io.

## 📋 Pre-Publishing Checklist

### 1. Verify Package Metadata

Check `Cargo.toml` has all required fields:

```toml
[package]
name = "kraky"
version = "0.1.0"           # Semantic versioning
edition = "2021"
rust-version = "1.70"       # MSRV (Minimum Supported Rust Version)
license = "MIT"
authors = ["Your Name <email@example.com>"]
description = "..."         # Max 200 characters
documentation = "..."       # docs.rs URL
homepage = "..."            # Project homepage
repository = "..."          # Git repository
keywords = [...]            # Max 5 keywords
categories = [...]          # From crates.io category list
readme = "README.md"
exclude = [...]             # Files to exclude from package
```

### 2. Run Quality Checks

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-features -- -D warnings

# Run all tests
cargo test --all-features

# Check documentation builds
cargo doc --no-deps --all-features

# Build all examples
cargo build --examples --all-features

# Check package builds
cargo package --allow-dirty
```

### 3. Verify README

- [ ] Clear project description
- [ ] Installation instructions
- [ ] Usage examples
- [ ] Feature flags documented
- [ ] License mentioned
- [ ] Links work correctly

### 4. Verify LICENSE

- [ ] LICENSE file exists
- [ ] License matches Cargo.toml (MIT)
- [ ] Copyright year is correct

### 5. Update CHANGELOG.md

- [ ] Version number matches Cargo.toml
- [ ] Release date added
- [ ] All changes documented
- [ ] Links to GitHub release

---

## 🔐 Authentication

### First Time Setup

1. **Create crates.io account**: https://crates.io
2. **Get API token**: https://crates.io/me
3. **Login via cargo**:

```bash
cargo login
# Paste your API token when prompted
```

The token is saved to `~/.cargo/credentials.toml`

---

## 📦 Packaging

### Dry Run

Test packaging without publishing:

```bash
# Create package and check what will be included
cargo package --list

# Build the package (creates target/package/kraky-0.1.0.crate)
cargo package

# Verify the package
cd target/package
tar -tzf kraky-0.1.0.crate | less

# Test the packaged version
cargo install --path .
cargo uninstall kraky
```

### Common Issues

**Large package size:**
- Add files to `exclude` in Cargo.toml
- Remove unnecessary files

**Missing files:**
- Check `.gitignore` (cargo package uses git files by default)
- Explicitly include files with `include` in Cargo.toml

**Dependencies:**
- Ensure all dependencies have versions on crates.io
- No path dependencies allowed (except dev-dependencies)

---

## 🚀 Publishing

### Final Checks

```bash
# 1. Ensure working directory is clean
git status

# 2. Commit all changes
git add .
git commit -m "chore: prepare v0.1.0 release"

# 3. Create git tag
git tag -a v0.1.0 -m "Release v0.1.0"

# 4. Final package test
cargo package --allow-dirty

# 5. Publish (DRY RUN first!)
cargo publish --dry-run
```

### Actual Publish

```bash
# Publish to crates.io (CANNOT BE UNDONE!)
cargo publish

# Push commits and tags to GitHub
git push origin main
git push origin v0.1.0
```

### Post-Publish

1. **Create GitHub Release**:
   - Go to: https://github.com/sarptekin/kraky/releases
   - Click "Draft a new release"
   - Select tag: v0.1.0
   - Title: "Kraky v0.1.0"
   - Description: Copy from CHANGELOG.md
   - Attach binaries if applicable
   - Publish release

2. **Verify on crates.io**:
   - Check: https://crates.io/crates/kraky
   - Verify documentation builds: https://docs.rs/kraky

3. **Announce**:
   - Tweet about it (if you want)
   - Post in Rust community forums
   - Update project links

---

## 🔄 Publishing Updates

### Patch Release (0.1.0 → 0.1.1)

Bug fixes, no breaking changes:

```bash
# 1. Update version in Cargo.toml
version = "0.1.1"

# 2. Update CHANGELOG.md
## [0.1.1] - 2024-XX-XX
### Fixed
- Bug fix description

# 3. Commit and publish
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.1.1"
git tag -a v0.1.1 -m "Release v0.1.1"
cargo publish
git push origin main --tags
```

### Minor Release (0.1.0 → 0.2.0)

New features, backwards compatible:

```bash
# 1. Update version
version = "0.2.0"

# 2. Update CHANGELOG.md with new features
### Added
- New feature description

# 3. Commit and publish
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"
cargo publish
git push origin main --tags
```

### Major Release (0.1.0 → 1.0.0)

Breaking changes:

```bash
# 1. Update version
version = "1.0.0"

# 2. Update CHANGELOG.md with migration guide
### Breaking Changes
- Description of breaking changes
- Migration guide

# 3. Commit and publish
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 1.0.0"
git tag -a v1.0.0 -m "Release v1.0.0 - First stable release"
cargo publish
git push origin main --tags
```

---

## ⚠️ Important Notes

### Cannot Undo

**Once published, you CANNOT:**
- Delete a version
- Modify a published version
- Reuse a version number

**You CAN:**
- Yank a version (makes it unavailable for new projects)
- Publish a new version with fixes

### Yanking a Version

If you published a broken version:

```bash
# Mark version as broken (prevents new installs)
cargo yank --vers 0.1.0

# Un-yank if you made a mistake
cargo yank --vers 0.1.0 --undo
```

### Version Requirements

- Follow [Semantic Versioning](https://semver.org/)
- Breaking changes require major version bump
- New features require minor version bump
- Bug fixes require patch version bump

---

## 📊 Package Statistics

### After Publishing

Monitor your package:

- **Downloads**: https://crates.io/crates/kraky
- **Documentation**: https://docs.rs/kraky
- **Reverse Dependencies**: Shows who uses your crate
- **Version History**: All published versions

### Badges for README

Add these to your README:

```markdown
[![Crates.io](https://img.shields.io/crates/v/kraky.svg)](https://crates.io/crates/kraky)
[![Documentation](https://docs.rs/kraky/badge.svg)](https://docs.rs/kraky)
[![License](https://img.shields.io/crates/l/kraky.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/kraky.svg)](https://crates.io/crates/kraky)
```

---

## 🔧 Troubleshooting

### "crate name is already taken"

The name "kraky" must be available on crates.io. Check:
- https://crates.io/crates/kraky
- If taken, choose a different name in Cargo.toml

### "failed to verify package tarball"

The package you're trying to publish doesn't build. Fix:
- Run `cargo package` locally
- Extract and test: `cd target/package/kraky-0.1.0 && cargo build`
- Fix any errors

### "some files are missing"

Files in .gitignore are excluded. Solutions:
- Remove from .gitignore
- Add to `include` in Cargo.toml
- Ensure files are committed to git

### "documentation failed to build"

docs.rs builds fail. Test locally:
```bash
cargo doc --no-deps --all-features
```

---

## 📚 Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io](https://crates.io)
- [Semantic Versioning](https://semver.org/)
- [Cargo Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)

---

## ✅ Quick Reference

```bash
# Pre-publish checks
cargo fmt --all
cargo clippy --all-features
cargo test --all-features
cargo doc --no-deps --all-features
cargo package --list

# Publish
cargo publish --dry-run
cargo publish

# Post-publish
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin main --tags
```

Good luck with your release! 🚀
