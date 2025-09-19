#!/bin/bash

echo "🚀 Setting up Rust Image Converter development environment..."

# Rust 설치 확인
echo "🔍 Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust is already installed"
fi

# Rustup 설치 확인 및 설정
echo "🔍 Checking rustup installation..."
if ! command -v rustup &> /dev/null; then
    echo "❌ rustup not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ rustup is already installed"
fi

# WASM 타겟 설치
echo "🎯 Installing WASM target..."
rustup target add wasm32-unknown-unknown

# wasm-pack 설치 확인
echo "🔍 Checking wasm-pack installation..."
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing wasm-pack..."
    cargo install wasm-pack
else
    echo "✅ wasm-pack is already installed"
fi

# Node.js 설치 확인 (npm publish용)
echo "🔍 Checking Node.js installation..."
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js from https://nodejs.org/"
    echo "   Or install via Homebrew: brew install node"
    exit 1
else
    echo "✅ Node.js is already installed"
fi

# npm 설치 확인
echo "🔍 Checking npm installation..."
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install npm"
    exit 1
else
    echo "✅ npm is already installed"
fi

echo ""
echo "🎉 Setup complete! You can now run:"
echo "   ./build.sh    - Build the WASM package"
echo "   ./publish.sh  - Publish to npm"
echo ""
echo "📋 Installed tools:"
echo "   - Rust: $(rustc --version)"
echo "   - wasm-pack: $(wasm-pack --version)"
echo "   - Node.js: $(node --version)"
echo "   - npm: $(npm --version)"
echo "   - WASM target: $(rustup target list --installed | grep wasm32-unknown-unknown || echo 'Not installed')"
