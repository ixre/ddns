#!/usr/bin/env bash
#docker pull ekidd/rust-musl-builder
docker run --rm -it -v $(pwd):/home/rust/src ekidd/rust-musl-builder \
cargo build --release --target x86_64-unknown-linux-musl

target=$(pwd)/dist/ddns
rm -rf dist && mkdir -p ${target}
cp ddns_default.conf ${target}/ddns.conf
cp target/x86_64-unknown-linux-musl/release/ddns ${target}
cd ${target}/../ && tar cvzf ddns-linux-x86_64.tar.gz ddns
cd - > /dev/null
