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
pub struct PublicipResourceFragment {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "public_ip_address", skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(rename = "private_ip_address", skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "port_id", skip_serializing_if = "Option::is_none")]
    pub port_id: Option<String>,
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "bandwidth_id", skip_serializing_if = "Option::is_none")]
    pub bandwidth_id: Option<String>,
    #[serde(
        rename = "bandwidth_share_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub bandwidth_share_type: Option<String>,
    #[serde(rename = "bandwidth_size", skip_serializing_if = "Option::is_none")]
    pub bandwidth_size: Option<i32>,
    #[serde(rename = "bandwidth_name", skip_serializing_if = "Option::is_none")]
    pub bandwidth_name: Option<String>,
}

impl PublicipResourceFragment {
    pub fn new() -> PublicipResourceFragment {
        PublicipResourceFragment {
            profile: None,
            _type: None,
            public_ip_address: None,
            private_ip_address: None,
            port_id: None,
            create_time: None,
            bandwidth_id: None,
            bandwidth_share_type: None,
            bandwidth_size: None,
            bandwidth_name: None,
        }
    }
}
