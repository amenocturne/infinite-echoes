compiled_wasm := "target/wasm32-unknown-unknown/release/game.wasm"
game_js := "dist/game.js"

build-game:
  cargo build --package game --release --target wasm32-unknown-unknown
  wasm-bindgen {{compiled_wasm}} --out-dir dist --target web
  sed -i '' "s/import \* as __wbg_star0 from 'env';//" {{game_js}}
  sed -i '' "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" {{game_js}}
  sed -i '' "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" {{game_js}}
  sed -i '' "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" {{game_js}}

build-server:
  cargo build --package server --release

build:build-game build-server

run: build download-runtime
  cargo run --package server --release

download-runtime:
	if [ ! -f "./web/miniquad_runtime.js" ]; then wget "https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"; mv mq_js_bundle.js ./web/miniquad_runtime.js; else echo "File exists, skipping"; fi
