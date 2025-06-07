compiled_wasm := "target/wasm32-unknown-unknown/release/game.wasm"
dist_dir := "dist"
game_js := dist_dir + "/game.js"
deploy_dir := "deploy"
target_dir := "target"

# Web Client
web_client_dir := "web_client"
web_client_build_dir := web_client_dir + "/build"
web_client_modules_dir := web_client_dir + "/node_modules"
web_client_src_dir := web_client_dir + "/src" # This will be the new location for copied contracts
web_client_contracts_dir := web_client_src_dir + "/contracts" # This will be the new location for copied contracts

# Contracts
contracts_dir := "contracts"
contracts_build_dir := contracts_dir + "/build" # Original build output from blueprint
contracts_modules_dir := contracts_dir + "/node_modules"

############################# All #############################

clean:
  rm -rf {{dist_dir}}
  rm -rf {{deploy_dir}}
  rm -rf {{target_dir}}
  rm -rf {{contracts_build_dir}}
  rm -rf {{contracts_modules_dir}}
  rm -rf {{web_client_build_dir}}
  rm -rf {{web_client_modules_dir}}
  rm -rf {{web_client_contracts_dir}} # Clean the copied contracts too

############################# Game Only #############################

build:
  cargo build --package game --release --target wasm32-unknown-unknown
  mkdir -p {{dist_dir}} # Ensure dist directory exists
  wasm-bindgen {{compiled_wasm}} --out-dir dist --target web
  @if [ "$(uname)" = "Darwin" ]; then \
    sed -i '' "s#import \* as __wbg_star0 from 'env';##" {{game_js}}; \
    sed -i '' "s#let wasm;#let wasm; window.set_wasm = (w) => wasm = w;#" {{game_js}}; \
    sed -i '' "s#imports\['env'\] = __wbg_star0;#return imports.wbg\;#" {{game_js}}; \
    sed -i '' "s#const imports = __wbg_get_imports();#return __wbg_get_imports();#" {{game_js}}; \
  else \
    sed -i "s#import \* as __wbg_star0 from 'env';##" {{game_js}}; \
    sed -i "s#let wasm;#let wasm; window.set_wasm = (w) => wasm = w;#" {{game_js}}; \
    sed -i "s#imports\['env'\] = __wbg_star0;#return imports.wbg\;#" {{game_js}}; \
    sed -i "s#const imports = __wbg_get_imports();#return __wbg_get_imports();#" {{game_js}}; \
  fi

run: build download-runtime pack
  static-web-server --root {{deploy_dir}} --port 1234


download-runtime:
	if [ ! -f "./web/miniquad_runtime.js" ]; then wget "https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"; mv mq_js_bundle.js ./web/miniquad_runtime.js; else echo "File exists, skipping"; fi

# Pack all static content into a directory for deployment
pack: build download-runtime web-install-dependencies copy-wrappers web-build copy-static-content

pack-fast: build copy-static-content

copy-static-content:
  mkdir -p {{deploy_dir}}
  # Copy web files (excluding ton-api.js and ton-wallet.js)
  cp ./web/index.html {{deploy_dir}}/
  cp ./web/style.css {{deploy_dir}}/
  cp ./web/miniquad_runtime.js {{deploy_dir}}/ # miniquad_runtime.js stays in web/ and is copied
  cp ./web/tonconnect-manifest.json {{deploy_dir}}/
  # Copy game files
  cp ./dist/game_bg.wasm {{deploy_dir}}/
  cp ./dist/game.js {{deploy_dir}}/
  # Copy all resources
  mkdir -p {{deploy_dir}}/resources
  cp -R ./resources/* {{deploy_dir}}/resources/
  mkdir -p {{deploy_dir}}/{{web_client_dir}}
  cp  {{web_client_dir}}/dist/bundle.js {{deploy_dir}}/{{web_client_dir}}


############################ Contracts Only #############################3

deploy-contracts: build-contracts
  cd {{contracts_dir}}; npx blueprint run

test-contracts: build-contracts
  cd {{contracts_dir}}; npx blueprint test

build-contracts: install-contract-dependencies
  cd {{contracts_dir}}; npx blueprint build --all

install-contract-dependencies:
  cd {{contracts_dir}}; npm install


############################ Web Client #############################3


web-build:
  cd {{web_client_dir}}; npm run build

web-format:
  cd {{web_client_dir}}; npm run format

web-install-dependencies:
  cd {{web_client_dir}}; npm i

# Updated copy-wrappers recipe
copy-wrappers: build-contracts
  find {{contracts_build_dir}} -type f -name "*.ts" \
  | xargs -I {} bash -c \
  'mkdir -p "{{web_client_src_dir}}/$(dirname {})"; cp "{}" "{{web_client_src_dir}}/{}"'
