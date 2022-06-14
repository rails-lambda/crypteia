#!/bin/bash
set -e

rm -rfv bin/crypteia bin/crypteia.zip 

cargo build -p crypteia --bin crypteia --release --target x86_64-unknown-linux-musl

cp target/x86_64-unknown-linux-musl/release/crypteia bin/crypteia
strip bin/crypteia
chmod +x bin/crypteia

pushd bin
zip -r crypteia.zip crypteia
popd
