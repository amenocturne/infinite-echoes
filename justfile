compiled_wasm := "target/wasm32-unknown-unknown/release/game.wasm"
dist_dir := "dist"
game_js := dist_dir + "/game.js"
deploy_dir := "deploy"
target_dir := "target"

build:
  cargo build --package game --release --target wasm32-unknown-unknown
  wasm-bindgen {{compiled_wasm}} --out-dir dist --target web
  sed -i '' "s/import \* as __wbg_star0 from 'env';//" {{game_js}}
  sed -i '' "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" {{game_js}}
  sed -i '' "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" {{game_js}}
  sed -i '' "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" {{game_js}}

run: build download-runtime pack
  python3 -m http.server 1234 -d {{deploy_dir}}


download-runtime:
	if [ ! -f "./web/miniquad_runtime.js" ]; then wget "https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"; mv mq_js_bundle.js ./web/miniquad_runtime.js; else echo "File exists, skipping"; fi

# Pack all static content into a directory for deployment
pack: build download-runtime
  mkdir -p {{deploy_dir}}
  # Copy web files
  cp ./web/index.html {{deploy_dir}}/
  cp ./web/miniquad_runtime.js {{deploy_dir}}/
  cp ./web/ton-wallet.js {{deploy_dir}}/
  cp ./web/tonconnect-manifest.json {{deploy_dir}}/
  # Copy game files
  cp ./dist/game_bg.wasm {{deploy_dir}}/
  cp ./dist/game.js {{deploy_dir}}/
  # Copy all resources
  mkdir -p {{deploy_dir}}/resources
  cp -R ./resources/* {{deploy_dir}}/resources/

clean:
  rm -rf {{dist_dir}}
  rm -rf {{deploy_dir}}
  rm -rf {{target_dir}}
