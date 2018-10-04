#!/bin/sh

set -ex
cd "$(dirname $0)"

cargo +nightly build --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/gravity.wasm --out-dir .

npm install
npm run serve
