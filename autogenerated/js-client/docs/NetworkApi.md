# OpenServiceCloudApi.NetworkApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**networkResourceCreate**](NetworkApi.md#networkResourceCreate) | **POST** /v1alpha/network_resources | create a new network resource.
[**networkResourceDelete**](NetworkApi.md#networkResourceDelete) | **DELETE** /v1alpha/network_resources/{nr_id} | remove specified network resource.
[**networkResourceGet**](NetworkApi.md#networkResourceGet) | **GET** /v1alpha/network_resources/{nr_id} | get the information of network resource.



## networkResourceCreate

> NetworkResourceSpec networkResourceCreate(networkResourceCreateRequest)

create a new network resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.NetworkApi();
let networkResourceCreateRequest = new OpenServiceCloudApi.NetworkResourceCreateRequest(); // NetworkResourceCreateRequest | parameters for the requested network resource.
apiInstance.networkResourceCreate(networkResourceCreateRequest, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **networkResourceCreateRequest** | [**NetworkResourceCreateRequest**](NetworkResourceCreateRequest.md)| parameters for the requested network resource. | 

### Return type

[**NetworkResourceSpec**](NetworkResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## networkResourceDelete

> Object networkResourceDelete(nrId, cloudProvider)

remove specified network resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.NetworkApi();
let nrId = "nrId_example"; // String | uuid of network resource created
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
apiInstance.networkResourceDelete(nrId, cloudProvider, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nrId** | **String**| uuid of network resource created | 
 **cloudProvider** | **String**| cloud provider name | 

### Return type

**Object**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## networkResourceGet

> NetworkResourceSpec networkResourceGet(nrId, cloudProvider)

get the information of network resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.NetworkApi();
let nrId = "nrId_example"; // String | uuid of network resource created
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
apiInstance.networkResourceGet(nrId, cloudProvider, (error, data, response) => {
  if (error) {
    console.error(error);
  } else {
    console.log('API called successfully. Returned data: ' + data);
  }
});
```

### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nrId** | **String**| uuid of network resource created | 
 **cloudProvider** | **String**| cloud provider name | 

### Return type

[**NetworkResourceSpec**](NetworkResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

