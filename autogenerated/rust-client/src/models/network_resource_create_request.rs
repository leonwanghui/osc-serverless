/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceCreateRequest {
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: crate::models::CloudProviderInfo,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "vpc_request_fragment", skip_serializing_if = "Option::is_none")]
    pub vpc_request_fragment: Option<crate::models::SubnetRequestFragment>,
    #[serde(rename = "subnet_request_fragment", skip_serializing_if = "Option::is_none")]
    pub subnet_request_fragment: Option<crate::models::SubnetRequestFragment>,
    #[serde(rename = "publicip_request_fragment", skip_serializing_if = "Option::is_none")]
    pub publicip_request_fragment: Option<crate::models::PublicipRequestFragment>,
}

impl NetworkResourceCreateRequest {
    pub fn new(cloud_provider: crate::models::CloudProviderInfo, name: String) -> NetworkResourceCreateRequest {
        NetworkResourceCreateRequest {
            cloud_provider: cloud_provider,
            name: name,
            description: None,
            vpc_request_fragment: None,
            subnet_request_fragment: None,
            publicip_request_fragment: None,
        }
    }
}


