#!/bin/bash

# Get machine architecture
machine_arch=$(uname -m)

# Define architectures
architectures=("aarch64-apple-darwin" "x86_64-apple-darwin")

# Build for each architecture
for arch in "${architectures[@]}"; do
    cargo build --example rosetta --release --target=$arch --features "serde serde_json check_rosetta"
done

echo -e "My machine architecture: $machine_arch\n"

# Run example/rosetta for each architecture
for arch in "${architectures[@]}"; do
    echo -e "\033[1mRunning example/rosetta on $arch\033[0m"
    ./target/$arch/release/examples/rosetta
    echo
done
