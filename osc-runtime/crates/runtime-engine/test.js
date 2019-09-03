wasm = require('./pkg/runtime_engine');

var data = {
    kind: "ComputeResource",
    metadata: {
        operation: "create",
        cloud_provider: "hwcloud",
        labels: "test"
    },
    spec: {
        name: "nothing",
        size: 1
    }
};
var output = wasm.load_resource_to_js(data);
console.log(output);
