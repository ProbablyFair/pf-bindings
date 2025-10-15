# Release Process

## Prerequisites

1. **GitHub Personal Access Token**: `GH_TOKEN` with `repo` scope
2. **NPM Token**: `NPM_TOKEN` from npmjs.com with publishing permissions
3. **PyPI Token**: `PYPI_TOKEN` from pypi.org with publishing permissions

Add these as secrets in your GitHub repository settings.

## Automated Release (Recommended)

1. Update version numbers in:
   - `package.json` (version field)
   - `crates/pf-bindings-python/pyproject.toml` (version field)
   - `crates/pf-bindings-node/Cargo.toml` (version field)
   - `crates/pf-bindings-core/Cargo.toml` (version field)

2. Commit changes and create a git tag:
   ```bash
   git commit -m "chore: bump version to 0.1.0"
   git tag v0.1.0
   git push origin main --tags
   ```

3. GitHub Actions will automatically:
   - Run tests
   - Build for all platforms (macOS, Linux, Windows)
   - Publish to npm (`@probablyfair/pf-bindings`)
   - Publish to PyPI (`pf-bindings-python`)
   - Create GitHub Release with binaries

## Manual Release

### Node.js (npm)

```bash
# Build for current platform
npm run build

# Test locally
npm test

# Publish to npm
npm publish
```

### Python (PyPI)

```bash
cd crates/pf-bindings-python

# Build wheel
maturin build --release

# Upload to PyPI
maturin upload target/wheels/*.whl --username __token__ --password $PYPI_TOKEN
```

### Cross-platform builds

Use GitHub Actions or cross-compile manually:

```bash
# For each target platform
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
```

## Version Management

Follow semantic versioning (SemVer):
- `MAJOR.MINOR.PATCH`
- `0.1.0` - First release
- `0.1.1` - Patch release
- `0.2.0` - Minor release with new features
- `1.0.0` - Major stable release

## After Release

1. Update documentation
2. Create release notes in GitHub
3. Announce in appropriate channels
4. Monitor for issues