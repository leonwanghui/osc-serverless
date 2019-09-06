#!/bin/sh

set -e

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --profiling --out-dir ../bundler --target bundler
wasm-pack build --profiling --out-dir ../nodejs --target nodejs

# # Run the `wasm2js` tool from `binaryen`
# wasm2js ../bundler/runtime_engine_bg.wasm -o ../bundler/runtime_engine_bg.js

# # Update our JS shim to require the JS file instead
# sed -i 's/runtime_engine_bg.wasm/runtime_engine_bg.js/' ../bundler/runtime_engine.js
