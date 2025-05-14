build-server:
  cargo build --bin game --release --target wasm32-unknown-unknown

build-game:
  cargo build --bin server --release

build:build-game build-server

run: build-game
  cargo run --bin server --release
