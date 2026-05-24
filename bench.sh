#!/bin/bash

TARGET="https://x1nx3r.dev"
DURATION="10s"
CONNECTIONS=100

echo "🏎️  Preparing to drag race: $TARGET"
echo "========================================="

# Check for oha (Rust-based)
if command -v oha &> /dev/null; then
    echo "Using 'oha' (Rust HTTP load generator)..."
    oha -z $DURATION -c $CONNECTIONS "$TARGET"

# Check for wrk (C-based, highly concurrent)
elif command -v wrk &> /dev/null; then
    echo "Using 'wrk'..."
    wrk -t4 -c$CONNECTIONS -d$DURATION "$TARGET"

# Check for hey (Go-based)
elif command -v hey &> /dev/null; then
    echo "Using 'hey'..."
    hey -z $DURATION -c $CONNECTIONS "$TARGET"

else
    echo "❌ No modern benchmarking tool found!"
    echo "Please install one of the following:"
    echo "  - oha:  cargo install oha"
    echo "  - wrk:  sudo apt install wrk"
    echo "  - hey:  go install github.com/rakyll/hey@latest"
    exit 1
fi
