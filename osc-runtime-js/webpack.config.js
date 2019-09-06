const path = require('path');

module.exports = {
  entry: "./src/osc_runtime_bundler.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    publicPath: "/dist/",
    filename: "index.js",
  },
  mode: "development",
  node: {
    fs: "empty"
  }
};
