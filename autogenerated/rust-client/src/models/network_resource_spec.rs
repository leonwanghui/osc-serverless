/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// NetworkResourceSpec : Detailed info that indicates the type of backend cloud network services.


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceSpec {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "cloud_provider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<crate::models::CloudProviderInfo>,
    #[serde(rename = "vpc_fragment", skip_serializing_if = "Option::is_none")]
    pub vpc_fragment: Option<crate::models::VpcResourceFragment>,
    #[serde(rename = "subnet_fragment", skip_serializing_if = "Option::is_none")]
    pub subnet_fragment: Option<crate::models::SubnetResourceFragment>,
    #[serde(rename = "publicip_fragment", skip_serializing_if = "Option::is_none")]
    pub publicip_fragment: Option<crate::models::PublicipResourceFragment>,
}

impl NetworkResourceSpec {
    /// Detailed info that indicates the type of backend cloud network services.
    pub fn new() -> NetworkResourceSpec {
        NetworkResourceSpec {
            id: None,
            name: None,
            status: None,
            cloud_provider: None,
            vpc_fragment: None,
            subnet_fragment: None,
            publicip_fragment: None,
        }
    }
}


