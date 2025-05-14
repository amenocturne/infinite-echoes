build-game:
  cargo build --package game --release --target wasm32-unknown-unknown
  wasm-bindgen "target/wasm32-unknown-unknown/release/game.wasm" --out-dir dist --target web
  sed -i '' "s/import \* as __wbg_star0 from 'env';//" dist/"game".js
  sed -i '' "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/"game".js
  sed -i '' "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/"game".js
  sed -i '' "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" dist/"game".js

build-server:
  cargo build --package server --release

build:build-game build-server

run: build
  cargo run --package server --release
