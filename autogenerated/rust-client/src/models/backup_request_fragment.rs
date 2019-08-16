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
pub struct BackupRequestFragment {
    #[serde(rename = "volume_id")]
    pub volume_id: String,
    #[serde(rename = "snapshot_id")]
    pub snapshot_id: String,
}

impl BackupRequestFragment {
    pub fn new(volume_id: String, snapshot_id: String) -> BackupRequestFragment {
        BackupRequestFragment {
            volume_id: volume_id,
            snapshot_id: snapshot_id,
        }
    }
}


