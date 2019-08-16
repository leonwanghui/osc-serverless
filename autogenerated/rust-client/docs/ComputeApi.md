# \ComputeApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compute_resource_create**](ComputeApi.md#compute_resource_create) | **post** /v1alpha/compute_resources | create a new compute resource.
[**compute_resource_delete**](ComputeApi.md#compute_resource_delete) | **delete** /v1alpha/compute_resources/{cr_id} | remove specified compute resource.
[**compute_resource_get**](ComputeApi.md#compute_resource_get) | **get** /v1alpha/compute_resources/{cr_id} | get the information of compute resource.



## compute_resource_create

> crate::models::ComputeResourceSpec compute_resource_create(compute_resource_create_request)
create a new compute resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_resource_create_request** | [**ComputeResourceCreateRequest**](ComputeResourceCreateRequest.md) | parameters for the requested compute resource. | Required | 

### Return type

[**crate::models::ComputeResourceSpec**](ComputeResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compute_resource_delete

> serde_json::Value compute_resource_delete(cr_id, cloud_provider, delete_publicip, delete_volume)
remove specified compute resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cr_id** | **String** | uuid of compute resource created. | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 
**delete_publicip** | **bool** | check if to delete public ip |  | 
**delete_volume** | **bool** | check if to delete related volume |  | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compute_resource_get

> crate::models::ComputeResourceSpec compute_resource_get(cr_id, cloud_provider)
get the information of compute resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cr_id** | **String** | uuid of compute resource created. | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 

### Return type

[**crate::models::ComputeResourceSpec**](ComputeResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

