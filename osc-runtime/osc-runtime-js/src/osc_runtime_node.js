const engine = require("../../runtime-engine/nodejs/runtime_engine");
const req = require("./request");
const YAML = require('yamljs');
const path = require('path');

var filename = path.resolve(__dirname, "../../../osc-config/osc_request_compute.yaml")
var data = YAML.load(filename);
// Set the operation manually to pass the test case.
data.metadata.operation = 'create';
console.log(data);
var body = engine.load_resource_to_js(data);
console.log(body);

switch (body.kind) {
  case 'ComputeResource':
    switch (body.operation) {
      case 'create':
        req.computeResourceCreate(body.param);
        break;
      case "delete":
        req.computeResourceDelete(body.param.name, body.param.cloud_provider, body.param.optional);
        break;
      default:
        console.log('operation {' + body.operation + '} not supported!');
    };
    break;
  default:
    console.log('resource {' + body.kind + '} not supported!');
}
