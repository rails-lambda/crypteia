#!/bin/bash
set -e

rm -rfv bin 
mkdir -p bin/extensions

cargo build -p rust-parameters-lambda-extension --bin parameters-lambda-extension --release --target x86_64-unknown-linux-musl

cp target/x86_64-unknown-linux-musl/release/parameters-lambda-extension bin/extensions/parameters-lambda-extension
strip bin/extensions/parameters-lambda-extension
chmod +x bin/extensions/parameters-lambda-extension

pushd bin
zip -r extensions.zip .
# rm extensions -rfv
popd
