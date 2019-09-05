#!/bin/sh

set -e

wasm-pack build --profiling --out-dir ../../osc-runtime-js/bundler --target bundler
wasm-pack build --profiling --out-dir ../../osc-runtime-js/web --target web
wasm-pack build --profiling --out-dir ../../osc-runtime-js/nodejs --target nodejs
