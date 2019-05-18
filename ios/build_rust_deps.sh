#!/bin/sh

PATH=$PATH:$HOME/.cargo/bin
cd ../rust/game_bindings
cargo build --target aarch64-apple-ios --features metal