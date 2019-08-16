# CloudServerRequestFragment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_ref** | **String** |  | 
**flavor_ref** | **String** |  | 
**user_data** | **String** |  | [optional] 
**admin_pass** | **String** |  | [optional] 
**key_name** | **String** |  | [optional] 
**vpcid** | **String** |  | 
**nics** | [**Vec<crate::models::CloudServerRequestFragmentNics>**](CloudServerRequestFragment_nics.md) |  | 
**ip_address** | **String** |  | [optional] 
**publicip** | [***crate::models::CloudServerRequestFragmentPublicip**](CloudServerRequestFragment_publicip.md) |  | [optional] 
**count** | **i32** |  | [optional] [default to 1]
**is_auto_rename** | **bool** |  | [optional] [default to true]
**root_volume** | [***crate::models::CloudServerRequestFragmentRootVolume**](CloudServerRequestFragment_root_volume.md) |  | 
**security_groups** | [**Vec<crate::models::CloudServerRequestFragmentSecurityGroups>**](CloudServerRequestFragment_security_groups.md) |  | [optional] 
**metadata** | [***serde_json::Value**](.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


