const rust =
  import ('../bundler');
import * as YAML from 'yamljs';
import * as req from './request_bundler';

let strdata = `
kind: ComputeResource
metadata:
  cloud_provider:
    name: huaweicloud
  labels:
    app: etcd
spec:
  name: newserver
  description: This is a new server for testing
  availability_zone: cn-north-1a # 华北-北京一
  cloud_provider: # Deprecated, please use metadata
    name: huaweicloud
  cloud_server_request_fragment:
    imageRef: f1dd2272-7041-479e-9663-646632b6ac00 # Ubuntu 16.04 server 64bit
    root_volume:
      volumetype: SATA
      size: 40
    flavorRef: t6.small.1 # 1vCPUs | 1GB
    key_name: KeyPair-2e4a
    user_data: "/etc/osc-config/app_install_etcd.sh"
    vpcid: e86bd162-8136-4227-a076-825161c95d29 # vpc-default | 192.168.0.0/16
    nics:
      - subnet_id: ff5330b5-c6f2-4983-8ba1-3fc81fe816e9 # subnet-default | 192.168.0.1
        # ip_address: x.x.x.x
    publicip:
      # id: 501e1f51-b00c-4d88-a586-29948c43910b
      eip: # create a new public ip
        ip_type: 5_sbgp
        bandwidth:
          name: newbandwidth
          sharetype: PER
          size: 1
    security_groups:
      - id: 86bd6416-3d4b-45d3-bf6e-605b9326015a # Sys-default
    count: 1
`
let data = YAML.parse(strdata);
// Set the operation manually to pass the test case.
data.metadata.operation = 'create';

rust
  .then(m => {
    let body = m.load_resource_to_js(data);
    console.log(body);
    switch (body.kind) {
      case 'ComputeResource':
        switch (body.operation) {
          case 'create':
            req.compute_resource_create(body.params);
            break;
          case "delete":
            req.compute_resource_delete(body.params.name, body.params.cloud_provider, body.params.optional);
            break;
          default:
            console.log('operation {' + body.operation + '} not supported!');
        };
        break;
      case 'StorageResource':
        switch (body.operation) {
          case 'create':
            req.storage_resource_create(body.params);
            break;
          case "delete":
            req.storage_resource_delete(body.params.name, body.params.cloud_provider);
            break;
          default:
            console.log('operation {' + body.operation + '} not supported!');
        };
        break;
      case 'NetworkResource':
        switch (body.operation) {
          case 'create':
            req.network_resource_create(body.params);
            break;
          case "delete":
            req.network_resource_delete(body.params.name, body.params.cloud_provider);
            break;
          default:
            console.log('operation {' + body.operation + '} not supported!');
        };
        break;
      default:
        console.log('resource {' + body.kind + '} not supported!');
    }
  })
  .catch(console.error);
