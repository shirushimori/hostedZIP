#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
GUI_DIR="$ROOT_DIR/src/web_GUI"
OUTPUT_DIR="$ROOT_DIR/build"
APP_NAME="openhub"

# Detect current OS
case "$(uname -s)" in
  Linux)  CURRENT_OS="linux" ;;
  MINGW*|MSYS*|CYGWIN*) CURRENT_OS="windows" ;;
  Darwin) CURRENT_OS="macos" ;;
  *)      CURRENT_OS="unknown" ;;
esac

echo "=== OpenHub Builder ==="
echo "Current OS: $CURRENT_OS"
echo ""
echo "Select target platform:"
echo "  1) Linux  (native)"
echo "  2) Windows  (experimental)"
read -rp "Choice [1/2]: " choice

case "$choice" in
  1|"")
    PLATFORM="linux"
    RUST_TARGET=""
    BIN_NAME="$APP_NAME"
    BIN_SRC="$ROOT_DIR/target/release/$APP_NAME"
    ;;
  2)
    PLATFORM="windows"
    RUST_TARGET="--target x86_64-pc-windows-gnu"
    BIN_NAME="${APP_NAME}.exe"
    BIN_SRC="$ROOT_DIR/target/x86_64-pc-windows-gnu/release/$BIN_NAME"
    ;;
  *)
    echo "Invalid choice"; exit 1
    ;;
esac

[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

# Ensure target toolchain is installed for cross-compile
if [ "$PLATFORM" != "linux" ] && [ "$CURRENT_OS" = "linux" ]; then
  echo ""
  echo "=== Installing Windows cross-compile target ==="
  rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
fi

echo ""
echo "=== Building frontend ==="
(cd "$GUI_DIR" && pnpm install && pnpm build)

echo ""
echo "=== Building Rust backend ($PLATFORM) ==="
(cd "$ROOT_DIR" && cargo build --release $RUST_TARGET)

echo ""
echo "=== Deploying to $OUTPUT_DIR ==="
rm -rf "$OUTPUT_DIR/services" "$OUTPUT_DIR/web" "$OUTPUT_DIR/data"
mkdir -p "$OUTPUT_DIR/services"
mkdir -p "$OUTPUT_DIR/web"

# Copy binary
if [ -f "$BIN_SRC" ]; then
  cp "$BIN_SRC" "$OUTPUT_DIR/"
  echo "Binary copied: $BIN_SRC → $OUTPUT_DIR/"
else
  echo "WARNING: Binary not found at $BIN_SRC"
fi

# Copy web frontend
cp -r "$GUI_DIR/dist/"* "$OUTPUT_DIR/web/"
echo "Web files copied"

# Copy data
if [ -d "$ROOT_DIR/data" ]; then
  mkdir -p "$OUTPUT_DIR/data"
  cp -r "$ROOT_DIR/data/"* "$OUTPUT_DIR/data/"
  echo "Data files copied"
fi

echo ""
echo "=== Build Complete ==="
echo "Output: $OUTPUT_DIR"
echo ""

# Auto-run if the built binary matches the current OS
CAN_RUN=false
if [ "$PLATFORM" = "linux" ] && [ "$CURRENT_OS" = "linux" ]; then
  CAN_RUN=true
elif [ "$PLATFORM" = "windows" ] && [ "$CURRENT_OS" = "windows" ]; then
  CAN_RUN=true
fi

if [ "$CAN_RUN" = true ]; then
  echo "Launching $APP_NAME..."
  echo ""
  cd "$OUTPUT_DIR" && exec "./$BIN_NAME"
else
  echo "Built for $PLATFORM — cannot run on $CURRENT_OS"
  echo "To run manually:"
  echo "  cd $OUTPUT_DIR && ./$BIN_NAME"
  echo ""
  echo "For development (Vite hot-reload + Rust API):"
  echo "  cd src/web_GUI && pnpm dev &"
  echo "  cargo run -- http://localhost:5173"
fi
