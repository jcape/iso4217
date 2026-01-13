#!/bin/bash

rustup toolchain install nightly
rustup toolchain install stable
rustup component add --toolchain stable rustfmt

mkdir -p /workspaces/iso4217/.cache/cargo
ln -sf /usr/local/cargo/bin /workspaces/iso4217/.cache/cargo/

cargo binstall -q -y --force prek
cargo binstall -q -y --force action-validator
cargo binstall -q -y --force cargo-deny
cargo binstall -q -y --force cargo-nextest
cargo binstall -q -y --force cargo-llvm-cov

pushd /workspaces/iso4217 >/dev/null
prek install -f >/dev/null
popd >/dev/null
