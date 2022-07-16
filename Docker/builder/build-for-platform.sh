#! /bin/sh
# Builds using the correct target for the current docker platform.

set -e

echo "Target Arch is: ${TARGETARCH}"
echo "Target Platform is: ${TARGETPLATFORM}"
case ${TARGETARCH} in
         "amd64")  export RUST_TARGET=x86_64-unknown-linux-musl;;
         "arm64")  export RUST_TARGET=aarch64-unknown-linux-musl;;
         *) export RUST_TARGET=x86_64-unknown-linux-musl;;
esac
echo "Target is ${RUST_TARGET}"

rustup target add $RUST_TARGET
cargo build --target $RUST_TARGET --release
cp target/$RUST_TARGET/release/dd-metric /dd-metric
chmod +x /dd-metric
