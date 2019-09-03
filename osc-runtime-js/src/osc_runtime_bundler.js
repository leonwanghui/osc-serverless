import load_resource_to_js from "../runtime-engine/bundler/runtime_engine.js";
// var OpenServiceCloudApi = import('../autogenerated/js-client/src/index');
// var defaultClient = OpenServiceCloudApi.ApiClient.instance;

// // Configure HTTP basic authorization: basicAuth
// var basicAuth = defaultClient.authentications['basicAuth'];
// basicAuth.username = 'YOUR USERNAME'
// basicAuth.password = 'YOUR PASSWORD'

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
var output = load_resource_to_js(data);
console.log(output);
var jsonData = JSON.stringify(output);

// var callback = function(error, data, response) {
//     if (error) {
//       console.error(error);
//     } else {
//       console.log('API called successfully. Returned data: ' + data);
//     }
// };

// switch (output.kind) {
//     case 'ComputeResource':
//         var api = new OpenServiceCloudApi.ComputeApi();
//         switch (output.operation) {
//             case 'create':
//                 var computeResourceCreateRequest = new OpenServiceCloudApi.ComputeResourceCreateRequest(); // {ComputeResourceCreateRequest} parameters for the requested compute resource.
//                 computeResourceCreateRequest = JSON.parse(jsonData);
//                 api.computeResourceCreate(computeResourceCreateRequest, callback);
//                 break;
//             case "delete":
//                 api.computeResourceDelete(output.crId, output.cloud_provider, {}, callback);
//                 break;
//             default:
//                 break;
//         };
//         break;
//     default:
//         break;
// }

// var f = document.getElementById('test-file-upload');
// var filename = f.value; // 'C:\fakepath\test.png'
// if (!filename || !(filename.endsWith('.jpg') || filename.endsWith('.png') || filename.endsWith('.gif'))) {
//     alert('Can only upload image file.');
//     return false;
// }