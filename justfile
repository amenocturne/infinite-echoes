build-game:
  cargo build --package game --release --target wasm32-unknown-unknown

build-server:
  cargo build --package server --release

build:build-game build-server

run: build
  cargo run --package server --release
