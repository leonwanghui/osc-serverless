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
pub struct BlockVolumeResourceFragment {
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(
        rename = "os-vol-host-attr:host",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_vol_host_attrhost: Option<String>,
    #[serde(rename = "multiattach", skip_serializing_if = "Option::is_none")]
    pub multiattach: Option<bool>,
    #[serde(
        rename = "dedicated_storage_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub dedicated_storage_id: Option<String>,
    #[serde(
        rename = "dedicated_storage_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub dedicated_storage_name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::BlockVolumeResourceFragmentAttachments>>,
}

impl BlockVolumeResourceFragment {
    pub fn new() -> BlockVolumeResourceFragment {
        BlockVolumeResourceFragment {
            volume_type: None,
            metadata: None,
            os_vol_host_attrhost: None,
            multiattach: None,
            dedicated_storage_id: None,
            dedicated_storage_name: None,
            tags: None,
            attachments: None,
        }
    }
}
