import init, { load_resource_to_js } from "../../runtime-engine/web/runtime_engine.js";

init();

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
var jsonData = JSON.stringify(data);
var output = load_resource_to_js(jsonData);
console.log(output);
