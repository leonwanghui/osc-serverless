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
pub struct FileShareResourceFragmentExportLocations {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "share_instance_id", skip_serializing_if = "Option::is_none")]
    pub share_instance_id: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "preferred", skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
}

impl FileShareResourceFragmentExportLocations {
    pub fn new() -> FileShareResourceFragmentExportLocations {
        FileShareResourceFragmentExportLocations {
            id: None,
            share_instance_id: None,
            path: None,
            preferred: None,
        }
    }
}


