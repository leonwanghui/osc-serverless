/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * The version of the OpenAPI document: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// StorageResourceSpec : Detailed info that indicates the type of backend cloud storage services.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageResourceSpec {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "cloud_provider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<crate::models::CloudProviderInfo>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "availability_zone", skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(
        rename = "block_volume_fragment",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_volume_fragment: Option<crate::models::BlockVolumeResourceFragment>,
    #[serde(
        rename = "file_share_fragment",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_share_fragment: Option<crate::models::FileShareResourceFragment>,
    #[serde(rename = "backup_fragment", skip_serializing_if = "Option::is_none")]
    pub backup_fragment: Option<crate::models::BackupResourceFragment>,
}

impl StorageResourceSpec {
    /// Detailed info that indicates the type of backend cloud storage services.
    pub fn new() -> StorageResourceSpec {
        StorageResourceSpec {
            id: None,
            name: None,
            description: None,
            created_at: None,
            updated_at: None,
            cloud_provider: None,
            status: None,
            availability_zone: None,
            size: None,
            block_volume_fragment: None,
            file_share_fragment: None,
            backup_fragment: None,
        }
    }
}
