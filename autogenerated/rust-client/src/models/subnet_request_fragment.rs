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
pub struct SubnetRequestFragment {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "cidr")]
    pub cidr: String,
    #[serde(rename = "gateway_ip")]
    pub gateway_ip: String,
    #[serde(rename = "vpc_id")]
    pub vpc_id: String,
    #[serde(rename = "dhcp_enable", skip_serializing_if = "Option::is_none")]
    pub dhcp_enable: Option<bool>,
    #[serde(rename = "primary_dns", skip_serializing_if = "Option::is_none")]
    pub primary_dns: Option<String>,
    #[serde(rename = "availability_zone", skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}

impl SubnetRequestFragment {
    pub fn new(name: String, cidr: String, gateway_ip: String, vpc_id: String) -> SubnetRequestFragment {
        SubnetRequestFragment {
            name: name,
            cidr: cidr,
            gateway_ip: gateway_ip,
            vpc_id: vpc_id,
            dhcp_enable: None,
            primary_dns: None,
            availability_zone: None,
        }
    }
}


