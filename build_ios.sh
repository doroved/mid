#!/bin/bash

set -e

# Define variables
LIB_NAME="libmid"
HEADER_DIR="include"
HEADER_FILE="$HEADER_DIR/mid.h"

# 1. Generate C Header
echo "Generating C header..."
mkdir -p "$HEADER_DIR"

cat > "$HEADER_FILE" <<EOF
#ifndef MID_H
#define MID_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get machine ID hash using the provided service name.
 * The returned string must be freed using mid_free_string.
 * Returns NULL on error.
 */
char* mid_get(const char* service_name);

/**
 * Free the string returned by mid_get.
 */
void mid_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* MID_H */
EOF

# Generate module.modulemap
cat > "$HEADER_DIR/module.modulemap" <<EOF
module Mid {
    header "mid.h"
    export *
}
EOF

# 2. Add targets if missing
echo "Checking Rust targets..."
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

# 3. Build for targets
echo "Building for iOS Device (arm64)..."
cargo build --release --target aarch64-apple-ios

echo "Building for iOS Simulator (arm64)..."
cargo build --release --target aarch64-apple-ios-sim

echo "Building for iOS Simulator (x86_64)..."
cargo build --release --target x86_64-apple-ios

# 4. Create universal library for Simulator
echo "Lipo: Creating universal simulator library..."
mkdir -p target/universal-sim
lipo -create \
    target/aarch64-apple-ios-sim/release/$LIB_NAME.a \
    target/x86_64-apple-ios/release/$LIB_NAME.a \
    -output target/universal-sim/$LIB_NAME.a

# 5. Create XCFramework
echo "Creating XCFramework..."
rm -rf Mid.xcframework

xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/$LIB_NAME.a \
    -headers "$HEADER_DIR" \
    -library target/universal-sim/$LIB_NAME.a \
    -headers "$HEADER_DIR" \
    -output Mid.xcframework

echo "✅ Success! Mid.xcframework created."
