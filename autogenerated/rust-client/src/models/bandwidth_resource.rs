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
pub struct BandwidthResource {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sharetype")]
    pub sharetype: Sharetype,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl BandwidthResource {
    pub fn new(sharetype: Sharetype) -> BandwidthResource {
        BandwidthResource {
            name: None,
            sharetype: sharetype,
            size: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Sharetype {
    #[serde(rename = "PER")]
    PER,
    #[serde(rename = "WHOLE")]
    WHOLE,
}

