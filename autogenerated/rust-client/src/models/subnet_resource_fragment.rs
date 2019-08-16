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
pub struct SubnetResourceFragment {
    #[serde(rename = "cidr", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "gateway_ip", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    #[serde(rename = "vpc_id", skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "dhcp_enable", skip_serializing_if = "Option::is_none")]
    pub dhcp_enable: Option<bool>,
    #[serde(rename = "primary_dns", skip_serializing_if = "Option::is_none")]
    pub primary_dns: Option<String>,
    #[serde(rename = "availability_zone", skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}

impl SubnetResourceFragment {
    pub fn new() -> SubnetResourceFragment {
        SubnetResourceFragment {
            cidr: None,
            gateway_ip: None,
            vpc_id: None,
            dhcp_enable: None,
            primary_dns: None,
            availability_zone: None,
        }
    }
}


