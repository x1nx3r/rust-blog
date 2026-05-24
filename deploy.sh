#!/bin/bash
# Local to Remote Deployment Orchestrator for Rust Bare-Metal

# Production server details (Update with actual user/host)
SERVER="inikah-my-admin@103.93.163.44"
APP_ROOT="/var/www/rust-blog"
TIMESTAMP=$(date +%Y%m%d%H%M%S)
RELEASE_DIR="$APP_ROOT/releases/$TIMESTAMP"

echo "-> 1. Compiling CSS and Rust binary locally (Offloading CPU/RAM from the potato)..."
# Build Tailwind
make css-prod
# Build the release binary. We build natively assuming local architecture matches the server (Linux x86_64).
# If building from a Mac, you'd cross-compile with: cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release

echo "-> 2. Pushing the monolithic binary to the bare-metal server..."
# Ensure the releases directory exists
ssh "$SERVER" "mkdir -p $RELEASE_DIR"

# We only need to push ONE file. The CSS, HTML, and assets are already fused inside the binary.
rsync -avz --progress target/release/rust-blog "$SERVER:$RELEASE_DIR/rust-blog"

# Copy the server-side Makefile to the release directory
rsync -avz --progress Makefile.server "$SERVER:$RELEASE_DIR/Makefile"

echo "-> 3. Triggering atomic shift and systemd restart..."
# Execute the Makefile on the server to swap symlinks and compress old releases
ssh "$SERVER" "cd $RELEASE_DIR && make -f Makefile deploy"

echo "✅ Deployment complete. Presses are rolling."
