const engine = require("../nodejs/runtime_engine");
const req = require("./request");
const YAML = require('yamljs');
const path = require('path');

var filename = path.resolve(__dirname, "../../osc-config/osc_request_compute.yaml")
var data = YAML.load(filename);
// Set the operation manually to pass the test case.
data.metadata.operation = 'create';
var body = engine.load_resource_to_js(data);

switch (body.kind) {
  case 'ComputeResource':
    switch (body.operation) {
      case 'create':
        req.computeResourceCreate(body.params);
        break;
      case "delete":
        req.computeResourceDelete(body.params.name, body.params.cloud_provider, body.params.optional);
        break;
      default:
        console.log('operation {' + body.operation + '} not supported!');
    };
    break;
  case 'StorageResource':
    switch (body.operation) {
      case 'create':
        req.storageResourceCreate(body.params);
        break;
      case "delete":
        req.storageResourceDelete(body.params.name, body.params.cloud_provider);
        break;
      default:
        console.log('operation {' + body.operation + '} not supported!');
    };
    break;
  case 'NetworkResource':
    switch (body.operation) {
      case 'create':
        req.networkResourceCreate(body.params);
        break;
      case "delete":
        req.networkResourceDelete(body.params.name, body.params.cloud_provider);
        break;
      default:
        console.log('operation {' + body.operation + '} not supported!');
    };
    break;
  default:
    console.log('resource {' + body.kind + '} not supported!');
}
