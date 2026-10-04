#!/bin/bash

ARCH=$(arch)

# Install 1.88 toolchain (will not change unless we bump MSRV)
rustup toolchain install 1.88 --profile default

# Install cargo-binstall
pushd /tmp >/dev/null
curl -qsfL https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-${ARCH}-unknown-linux-gnu.tgz > /tmp/binstall.tar.gz
tar -zxf /tmp/binstall.tar.gz
install -Dpm 0755 /tmp/cargo-binstall /usr/local/cargo/bin/
rm -f /tmp/binstall.tar.gz /tmp/cargo-binstall

popd >/dev/null
