set -xue

rm -rf ./public/wasm/
pushd wasm
wasm-pack build --target web --out-dir ../public/wasm
popd
yarn dev # サーバーの実行
