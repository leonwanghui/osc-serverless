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
pub struct PublicipRequestFragment {
    #[serde(rename = "publicip")]
    pub publicip: crate::models::PublicipRequestFragmentPublicip,
    #[serde(rename = "bandwidth")]
    pub bandwidth: crate::models::BandwidthResource,
}

impl PublicipRequestFragment {
    pub fn new(publicip: crate::models::PublicipRequestFragmentPublicip, bandwidth: crate::models::BandwidthResource) -> PublicipRequestFragment {
        PublicipRequestFragment {
            publicip: publicip,
            bandwidth: bandwidth,
        }
    }
}


