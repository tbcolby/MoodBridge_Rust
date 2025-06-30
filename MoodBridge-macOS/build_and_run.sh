#!/bin/bash

# MoodBridge macOS App - Build and Run Script

set -e

echo "🧠 MoodBridge macOS App Builder"
echo "================================"

# Check if we're in the right directory
if [ ! -f "Package.swift" ]; then
    echo "❌ Error: Package.swift not found. Please run this script from the MoodBridge-macOS directory."
    exit 1
fi

# Check if Swift is installed
if ! command -v swift &> /dev/null; then
    echo "❌ Error: Swift is not installed or not in PATH."
    echo "Please install Xcode or Swift toolchain."
    exit 1
fi

echo "✅ Swift found: $(swift --version | head -n1)"

# Check if the Rust API is running
echo ""
echo "🔍 Checking if MoodBridge Rust API is running..."
if curl -s http://127.0.0.1:8000/api/health > /dev/null 2>&1; then
    echo "✅ MoodBridge API is running on http://127.0.0.1:8000"
else
    echo "⚠️  Warning: MoodBridge API doesn't seem to be running on http://127.0.0.1:8000"
    echo "   Please start the Rust API first:"
    echo "   cd ../  # Go to MoodBridge_Rust directory"
    echo "   cargo run"
    echo ""
    read -p "Continue anyway? (y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo ""
echo "📦 Resolving Swift package dependencies..."
swift package resolve

echo ""
echo "🔨 Building MoodBridge macOS app..."
swift build -c release

echo ""
echo "🚀 Launching MoodBridge macOS app..."
echo "   (Press Ctrl+C to stop the app)"
echo ""

# Run the app
swift run -c release MoodBridge
