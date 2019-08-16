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
pub struct CloudServerRequestFragmentPublicipEip {
    #[serde(rename = "ip_type")]
    pub ip_type: IpType,
    #[serde(rename = "bandwidth")]
    pub bandwidth: crate::models::BandwidthResource,
}

impl CloudServerRequestFragmentPublicipEip {
    pub fn new(ip_type: IpType, bandwidth: crate::models::BandwidthResource) -> CloudServerRequestFragmentPublicipEip {
        CloudServerRequestFragmentPublicipEip {
            ip_type: ip_type,
            bandwidth: bandwidth,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IpType {
    #[serde(rename = "5_bgp")]
    Bgp,
    #[serde(rename = "5_sbgp")]
    Sbgp,
}

