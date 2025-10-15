#!/bin/bash

set -e

# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.1.0

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

VERSION=$1

echo "Starting release for version $VERSION"

# Remove v prefix if present and update VERSION file
VERSION_NUMBER=$(echo $VERSION | sed 's/^v//')
echo $VERSION_NUMBER > VERSION

# Update versions in all files
echo "Updating version numbers..."

# Update package.json files
find packages/ -name "package.json" -exec sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION_NUMBER\"/" {} \;

# Update pyproject.toml
sed -i '' "s/version = \".*\"/version = \"$VERSION_NUMBER\"/" packages/python/pyproject.toml

# Update workspace Cargo.toml (only the workspace.package.version)
sed -i '' "s/^version = \".*\"$/version = \"$VERSION_NUMBER\"/" Cargo.toml

# Update individual crate Cargo.toml files (skip WASM which has special dependency requirements)
for crate in crates/*/Cargo.toml; do
  if [[ "$crate" != *"wasm"* ]]; then
    sed -i '' '/^name =/ { N; s/^version = .*/version = "'$VERSION_NUMBER'"/; }' "$crate"
  fi
done

# Update Java pom.xml
sed -i '' "s/<version>.*<\/version>/<version>$VERSION_NUMBER<\/version>/" packages/java/pom.xml

# Update C# project
sed -i '' "s/<Version>.*<\/Version>/<Version>$VERSION_NUMBER<\/Version>/" packages/csharp/PF.Bindings.csproj

# Update Swift Package.swift
sed -i '' "s/s.version = \".*\"/s.version = \"$VERSION_NUMBER\"/" packages/swift/Package.swift

# Update PHP - PHP extension doesn't use composer.json version
echo "  Updated PHP extension version constants"

# Update Dart pubspec.yaml
sed -i '' "s/version: .*/version: $VERSION_NUMBER/" packages/dart/pubspec.yaml

# Update WASM package.json
sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION_NUMBER\"/" packages/wasm/package.json

# Update Go go.mod
sed -i '' "s/github.com\/probablyfair\/pf-bindings-go v.*/github.com\/probablyfair\/pf-bindings-go v$VERSION_NUMBER/" packages/go/go.mod

# Update C/C++ headers
sed -i '' "s/const char \*pf_library_version();/const char \*pf_library_version();/" packages/c/pf_bindings.h
sed -i '' "s/return \"0.1.0\";/return \"$VERSION_NUMBER\";/" crates/pf-bindings-c/src/lib.rs

# Update PHP version constants
sed -i '' "s/const PF_BINDING_VERSION = \"0.1.0\";/const PF_BINDING_VERSION = \"$VERSION_NUMBER\";/" packages/php/php_pf_bindings.stub.php
sed -i '' "s/#define PHP_PF_BINDINGS_VERSION \"0.1.0\"/#define PHP_PF_BINDINGS_VERSION \"$VERSION_NUMBER\"/" packages/php/pf-bindings.h
sed -i '' "s/#define PF_BINDINGS_VERSION \"0.1.0\"/#define PF_BINDINGS_VERSION \"$VERSION_NUMBER\"/" packages/php/pf-bindings.c

# Update Rust WASM version
sed -i '' "s/return \"0.1.0\".to_string();/return \"$VERSION_NUMBER\".to_string();/" crates/pf-bindings-wasm/src/lib.rs

# Run tests (skip Python due to linking issues on some systems)
echo "Running tests..."
cargo test --workspace --exclude pf-bindings-python

# Build locally to verify
echo "Building locally..."
cargo build --release

# Create git tag
echo "Creating git tag..."
git add .
git commit -m "chore: bump version to $VERSION"
git tag "v$VERSION"

echo "Ready to release."
echo ""
echo "Next steps:"
echo "1. Review the changes: git diff HEAD~1"
echo "2. Push to GitHub: git push origin main --tags"
echo "3. GitHub Actions will automatically build and publish all packages"
echo ""
echo "Automation will publish to:"
echo "  ✅ NPM (Node.js/WebAPI)                   - @probablyfair/pf-bindings"
echo "  ✅ PyPI (Python)                          - pf-bindings-python"
echo "  ✅ NuGet (C#)                              - PF.Bindings"
echo "  ✅ GitHub Go modules                      - github.com/probablyfair/pf-bindings-go"
echo "  ✅ Maven Central (Java)                    - com.probablyfair:pf-bindings-java"
echo "  ✅ RubyGems (Ruby)                         - pf-bindings"
echo "  ✅ pub.dev (Dart/Flutter)                 - pf_bindings"
echo "  ✅ GitHub Release (native libraries & headers)"
echo "     - Native libraries (.dll, .so, .dylib)"
echo "     - C/C++ headers & libraries"
echo "     - PHP extension binaries"
echo "     - WebAssembly packages"
echo "     - Swift Package (downloadable)"
echo ""
echo "Or to release manually:"
echo "  npm publish                         # Node.js"
echo "  cd packages/python && maturin publish     # Python"
echo "  cd packages/csharp && dotnet pack && dotnet push  # C#"
echo "  cd packages/java && mvn deploy      # Java"
echo "  cd packages/ruby && gem push *.gem  # Ruby"
echo "  cd packages/dart && dart pub publish     # Dart"
echo "  cd php && phpize && make && make install  # PHP (local)"
echo ""
echo "Platform builds provided by GitHub Actions:"
echo "  Windows (x64)      Linux (x64, arm64)      macOS (x64, arm64)"
