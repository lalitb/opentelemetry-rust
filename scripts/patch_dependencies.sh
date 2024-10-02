#!/bin/bash

# Function to compare two version numbers
function version_lt() {
    [ "$(printf '%s\n' "$1" "$2" | sort -V | head -n1)" != "$2" ]
}

# Get the current Rust compiler version
rust_version=$(rustc --version | cut -d' ' -f2)

# Target version (Rust 1.71.1)
target_version="1.71.1"

  function patch_version() {
    local package=$1
    local new_version=$2
    local crate_path=$3
    local latest_version=$(cargo search --limit 1 $package | head -1 | cut -d'"' -f2)
    if [ -n "$crate_path" ]; then
        # If crate_path is provided, patch only for the specified crate
        echo "-->Patching $package in $crate_path from $latest_version to $new_version"
        cargo update -p $package:$latest_version --precise $new_version --manifest-path "$crate_path/Cargo.toml"
    else
        # If crate_path is not provided, patch for the entire workspace
        echo "Patching $package in the entire workspace from $latest_version to $new_version"
        cargo update -p $package:$latest_version --precise $new_version
    fi
  }

  patch_version cc 1.0.105
  patch_version url 2.5.0
if version_lt "$rust_version" "$target_version"; then
  patch_version tonic 0.12.3 "opentelemetry-sdk"
  patch_version hyper-rustls 0.27.2 # 0.27.3 needs rustc v1.70.0
  patch_version tokio-util 0.7.11 "opentelemetry-sdk" # 0.7.12 needs rustc v1.70.0
  patch_version tokio-stream 0.1.16 "opentelemetry-sdk" # 0.1.16 needs rustc v1.70.0
  patch_version tokio 1.38.0 "opentelemetry-sdk" # 1.39 needs msrv bump to rustc 1.70
fi
