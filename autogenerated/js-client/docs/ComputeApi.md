# OpenServiceCloudApi.ComputeApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**computeResourceCreate**](ComputeApi.md#computeResourceCreate) | **POST** /v1alpha/compute_resources | create a new compute resource.
[**computeResourceDelete**](ComputeApi.md#computeResourceDelete) | **DELETE** /v1alpha/compute_resources/{cr_id} | remove specified compute resource.
[**computeResourceGet**](ComputeApi.md#computeResourceGet) | **GET** /v1alpha/compute_resources/{cr_id} | get the information of compute resource.



## computeResourceCreate

> ComputeResourceSpec computeResourceCreate(computeResourceCreateRequest)

create a new compute resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.ComputeApi();
let computeResourceCreateRequest = new OpenServiceCloudApi.ComputeResourceCreateRequest(); // ComputeResourceCreateRequest | parameters for the requested compute resource.
apiInstance.computeResourceCreate(computeResourceCreateRequest, (error, data, response) => {
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
 **computeResourceCreateRequest** | [**ComputeResourceCreateRequest**](ComputeResourceCreateRequest.md)| parameters for the requested compute resource. | 

### Return type

[**ComputeResourceSpec**](ComputeResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## computeResourceDelete

> Object computeResourceDelete(crId, cloudProvider, opts)

remove specified compute resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.ComputeApi();
let crId = "crId_example"; // String | uuid of compute resource created.
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
let opts = {
  'deletePublicip': true, // Boolean | check if to delete public ip
  'deleteVolume': true // Boolean | check if to delete related volume
};
apiInstance.computeResourceDelete(crId, cloudProvider, opts, (error, data, response) => {
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
 **crId** | **String**| uuid of compute resource created. | 
 **cloudProvider** | **String**| cloud provider name | 
 **deletePublicip** | **Boolean**| check if to delete public ip | [optional] 
 **deleteVolume** | **Boolean**| check if to delete related volume | [optional] 

### Return type

**Object**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## computeResourceGet

> ComputeResourceSpec computeResourceGet(crId, cloudProvider)

get the information of compute resource.

### Example

```javascript
import OpenServiceCloudApi from 'open_service_cloud_api';
let defaultClient = OpenServiceCloudApi.ApiClient.instance;
// Configure HTTP basic authorization: basicAuth
let basicAuth = defaultClient.authentications['basicAuth'];
basicAuth.username = 'YOUR USERNAME';
basicAuth.password = 'YOUR PASSWORD';

let apiInstance = new OpenServiceCloudApi.ComputeApi();
let crId = "crId_example"; // String | uuid of compute resource created.
let cloudProvider = "cloudProvider_example"; // String | cloud provider name
apiInstance.computeResourceGet(crId, cloudProvider, (error, data, response) => {
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
 **crId** | **String**| uuid of compute resource created. | 
 **cloudProvider** | **String**| cloud provider name | 

### Return type

[**ComputeResourceSpec**](ComputeResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

