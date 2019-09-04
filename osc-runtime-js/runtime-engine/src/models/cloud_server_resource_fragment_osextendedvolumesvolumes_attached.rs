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
pub struct CloudServerResourceFragmentOsextendedvolumesvolumesAttached {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "delete_on_termination",
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_on_termination: Option<DeleteOnTermination>,
    #[serde(rename = "boolIndex", skip_serializing_if = "Option::is_none")]
    pub bool_index: Option<BoolIndex>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl CloudServerResourceFragmentOsextendedvolumesvolumesAttached {
    pub fn new() -> CloudServerResourceFragmentOsextendedvolumesvolumesAttached {
        CloudServerResourceFragmentOsextendedvolumesvolumesAttached {
            id: None,
            delete_on_termination: None,
            bool_index: None,
            device: None,
        }
    }
}

///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteOnTermination {
    #[serde(rename = "true")]
    _True,
    #[serde(rename = "false")]
    _False,
}
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BoolIndex {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "-1")]
    _1,
}
