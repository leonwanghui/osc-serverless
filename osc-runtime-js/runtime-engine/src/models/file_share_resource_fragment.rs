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
pub struct FileShareResourceFragment {
    #[serde(rename = "share_server_id", skip_serializing_if = "Option::is_none")]
    pub share_server_id: Option<String>,
    #[serde(rename = "share_network_id", skip_serializing_if = "Option::is_none")]
    pub share_network_id: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "share_proto", skip_serializing_if = "Option::is_none")]
    pub share_proto: Option<String>,
    #[serde(rename = "share_type_name", skip_serializing_if = "Option::is_none")]
    pub share_type_name: Option<String>,
    #[serde(rename = "share_type", skip_serializing_if = "Option::is_none")]
    pub share_type: Option<String>,
    #[serde(rename = "export_locations", skip_serializing_if = "Option::is_none")]
    pub export_locations: Option<Vec<crate::models::FileShareResourceFragmentExportLocations>>,
}

impl FileShareResourceFragment {
    pub fn new() -> FileShareResourceFragment {
        FileShareResourceFragment {
            share_server_id: None,
            share_network_id: None,
            metadata: None,
            host: None,
            share_proto: None,
            share_type_name: None,
            share_type: None,
            export_locations: None,
        }
    }
}
