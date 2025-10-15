default:
    @just --list

# Core
build-core:
    cargo build --release -p pf-bindings-core

test-core:
    cargo test -p pf-bindings-core

# Node.js/JavaScript
build-node:
    npm --prefix packages/node install
    npm --prefix packages/node run build

build-wasm:
    cd packages/wasm && wasm-pack build --target web --out-dir pkg

# Python
build-python:
    cd packages/python && maturin build --release

# Go
build-go:
    cd packages/go && go build ./...

# Java
build-java:
    cd packages/java && mvn compile

# C#
build-csharp:
    cd packages/csharp && dotnet build

# Swift
build-swift:
    cd packages/swift && swift build

# Build all
build-all:
    just build-core
    just build-node
    just build-python

# Release all
release-all:
    ./scripts/release.sh {{version}}
