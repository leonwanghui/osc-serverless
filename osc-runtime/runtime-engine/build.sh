#!/bin/sh

set -e

wasm-pack build --profiling --out-dir bundler --target bundler
wasm-pack build --profiling --out-dir web --target web
wasm-pack build --profiling --out-dir nodejs --target nodejs
