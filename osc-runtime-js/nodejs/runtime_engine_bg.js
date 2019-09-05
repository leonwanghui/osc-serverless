
const path = require('path').join(__dirname, 'runtime_engine_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./runtime_engine.js'] = require('./runtime_engine.js');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
