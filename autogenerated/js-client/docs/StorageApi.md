# OpenServiceCloudApi.StorageApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storageResourceCreate**](StorageApi.md#storageResourceCreate) | **POST** /v1alpha/storage_resources | create a new storage resource.
[**storageResourceDelete**](StorageApi.md#storageResourceDelete) | **DELETE** /v1alpha/storage_resources/{sr_id} | remove specified storage resource.
[**storageResourceGet**](StorageApi.md#storageResourceGet) | **GET** /v1alpha/storage_resources/{sr_id} | get the information of storage resource.



## storageResourceCreate

> StorageResourceSpec storageResourceCreate(storageResourceCreateRequest)

create a new storage resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.StorageApi();
let storageResourceCreateRequest = new OpenServiceCloudApi.StorageResourceCreateRequest(); // StorageResourceCreateRequest | parameters for the requested storage resource.
apiInstance.storageResourceCreate(storageResourceCreateRequest, (error, data, response) => {
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
 **storageResourceCreateRequest** | [**StorageResourceCreateRequest**](StorageResourceCreateRequest.md)| parameters for the requested storage resource. | 

### Return type

[**StorageResourceSpec**](StorageResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## storageResourceDelete

> Object storageResourceDelete(srId, cloudProvider)

remove specified storage resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.StorageApi();
let srId = "srId_example"; // String | uuid of storage resource created
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
apiInstance.storageResourceDelete(srId, cloudProvider, (error, data, response) => {
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
 **srId** | **String**| uuid of storage resource created | 
 **cloudProvider** | **String**| cloud provider name | 

### Return type

**Object**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## storageResourceGet

> StorageResourceSpec storageResourceGet(srId, cloudProvider)

get the information of storage resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.StorageApi();
let srId = "srId_example"; // String | uuid of storage resource created
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
apiInstance.storageResourceGet(srId, cloudProvider, (error, data, response) => {
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
 **srId** | **String**| uuid of storage resource created | 
 **cloudProvider** | **String**| cloud provider name | 

### Return type

[**StorageResourceSpec**](StorageResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

