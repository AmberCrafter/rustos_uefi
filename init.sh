#!/bin/bash

# install rust tool
# cargo install cargo-xbuild
# rustup component add rust-src
# rustup component add llvm-tools-preview
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# install qemu
sudo apt-get update
sudo apt-get install -y qemu-system-x86 gdb