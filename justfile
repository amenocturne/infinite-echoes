compiled_wasm := "target/wasm32-unknown-unknown/release/game.wasm"
dist_dir := "dist"
game_js := dist_dir + "/game.js"
deploy_dir := "deploy"
target_dir := "target"
ton_integration_dir := "ton_integration"

# Contracts
contracts_dir := "contracts"
contracts_build_dir := contracts_dir + "/build"
contracts_modules_dir := contracts_dir + "/node_modules"

############################# All #############################

clean:
  rm -rf {{dist_dir}}
  rm -rf {{deploy_dir}}
  rm -rf {{target_dir}}
  rm -rf {{contracts_build_dir}}
  rm -rf {{contracts_modules_dir}}

############################# Game Only #############################

build:
  cargo build --package game --release --target wasm32-unknown-unknown
  wasm-bindgen {{compiled_wasm}} --out-dir dist --target web
  sed -e "s/import \* as __wbg_star0 from 'env';//" {{game_js}} > {{game_js}}.tmp && mv {{game_js}}.tmp {{game_js}}
  sed -e "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" {{game_js}} > {{game_js}}.tmp && mv {{game_js}}.tmp {{game_js}}
  sed -e "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" {{game_js}} > {{game_js}}.tmp && mv {{game_js}}.tmp {{game_js}}
  sed -e "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" {{game_js}} > {{game_js}}.tmp && mv {{game_js}}.tmp {{game_js}}

run: build download-runtime pack
  python3 -m http.server 1234 -d {{deploy_dir}}


download-runtime:
	if [ ! -f "./web/miniquad_runtime.js" ]; then wget "https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"; mv mq_js_bundle.js ./web/miniquad_runtime.js; else echo "File exists, skipping"; fi

# Pack all static content into a directory for deployment
pack: build download-runtime
  mkdir -p {{deploy_dir}}
  # Copy web files
  cp ./web/* {{deploy_dir}}/
  # Copy game files
  cp ./dist/game_bg.wasm {{deploy_dir}}/
  cp ./dist/game.js {{deploy_dir}}/
  # Copy all resources
  mkdir -p {{deploy_dir}}/resources
  cp -R ./resources/* {{deploy_dir}}/resources/


############################ Contracts Only #############################3

deploy-contracts: build-contracts
  cd {{contracts_dir}}; npx blueprint run

test-contracts: build-contracts
  cd {{contracts_dir}}; npx blueprint test

build-contracts: install-contract-dependencies
  cd {{contracts_dir}}; npx blueprint build --all

install-contract-dependencies:
  cd {{contracts_dir}}; npm install


############################ Ton Integration #############################3


build-ton:
  cd {{ton_integration_dir}}; npm run build
