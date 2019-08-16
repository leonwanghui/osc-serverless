# \NetworkApi

All URIs are relative to *http://localhost:6106*

Method | HTTP request | Description
------------- | ------------- | -------------
[**network_resource_create**](NetworkApi.md#network_resource_create) | **post** /v1alpha/network_resources | create a new network resource.
[**network_resource_delete**](NetworkApi.md#network_resource_delete) | **delete** /v1alpha/network_resources/{nr_id} | remove specified network resource.
[**network_resource_get**](NetworkApi.md#network_resource_get) | **get** /v1alpha/network_resources/{nr_id} | get the information of network resource.



## network_resource_create

> crate::models::NetworkResourceSpec network_resource_create(network_resource_create_request)
create a new network resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_resource_create_request** | [**NetworkResourceCreateRequest**](NetworkResourceCreateRequest.md) | parameters for the requested network resource. | Required | 

### Return type

[**crate::models::NetworkResourceSpec**](NetworkResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_resource_delete

> serde_json::Value network_resource_delete(nr_id, cloud_provider)
remove specified network resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nr_id** | **String** | uuid of network resource created | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_resource_get

> crate::models::NetworkResourceSpec network_resource_get(nr_id, cloud_provider)
get the information of network resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nr_id** | **String** | uuid of network resource created | Required | 
**cloud_provider** | **String** | cloud provider name | Required | 

### Return type

[**crate::models::NetworkResourceSpec**](NetworkResourceSpec.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

