/*
 * Open Service Cloud API
 *
 * Open Service Cloud API to manage different backend cloud services.
 *
 * API version: 0.0.3
 * Contact: wanghui71leon@gmail.com
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

type SubnetRequestFragment struct {
	Name string `json:"name"`

	Cidr string `json:"cidr"`

	GatewayIp string `json:"gateway_ip"`

	VpcId string `json:"vpc_id"`

	DhcpEnable bool `json:"dhcp_enable,omitempty"`

	PrimaryDns string `json:"primary_dns,omitempty"`

	AvailabilityZone string `json:"availability_zone,omitempty"`
}
