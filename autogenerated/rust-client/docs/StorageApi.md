# \StorageApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_resource_create**](StorageApi.md#storage_resource_create) | **post** /v1alpha/storage_resources | create a new storage resource.
[**storage_resource_delete**](StorageApi.md#storage_resource_delete) | **delete** /v1alpha/storage_resources/{sr_id} | remove specified storage resource.
[**storage_resource_get**](StorageApi.md#storage_resource_get) | **get** /v1alpha/storage_resources/{sr_id} | get the information of storage resource.



## storage_resource_create

> crate::models::StorageResourceSpec storage_resource_create(storage_resource_create_request)
create a new storage resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_resource_create_request** | [**StorageResourceCreateRequest**](StorageResourceCreateRequest.md) | parameters for the requested storage resource. | Required | 

### Return type

[**crate::models::StorageResourceSpec**](StorageResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_resource_delete

> serde_json::Value storage_resource_delete(sr_id, cloud_provider)
remove specified storage resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sr_id** | **String** | uuid of storage resource created | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_resource_get

> crate::models::StorageResourceSpec storage_resource_get(sr_id, cloud_provider)
get the information of storage resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sr_id** | **String** | uuid of storage resource created | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 

### Return type

[**crate::models::StorageResourceSpec**](StorageResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

