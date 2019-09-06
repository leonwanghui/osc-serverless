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
pub struct CloudServerRequestFragmentSecurityGroups {
    #[serde(rename = "id")]
    pub id: String,
}

impl CloudServerRequestFragmentSecurityGroups {
    pub fn new(id: String) -> CloudServerRequestFragmentSecurityGroups {
        CloudServerRequestFragmentSecurityGroups { id: id }
    }
}