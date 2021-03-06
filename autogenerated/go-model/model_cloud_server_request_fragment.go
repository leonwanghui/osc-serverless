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

type CloudServerRequestFragment struct {
	ImageRef string `json:"imageRef"`

	FlavorRef string `json:"flavorRef"`

	UserData string `json:"user_data,omitempty"`

	AdminPass string `json:"adminPass,omitempty"`

	KeyName string `json:"key_name,omitempty"`

	Vpcid string `json:"vpcid"`

	Nics []CloudServerRequestFragmentNics `json:"nics"`

	IpAddress string `json:"ip_address,omitempty"`

	Publicip CloudServerRequestFragmentPublicip `json:"publicip,omitempty"`

	Count int32 `json:"count,omitempty"`

	IsAutoRename bool `json:"isAutoRename,omitempty"`

	RootVolume CloudServerRequestFragmentRootVolume `json:"root_volume"`

	SecurityGroups []CloudServerRequestFragmentSecurityGroups `json:"security_groups,omitempty"`

	Metadata map[string]interface{} `json:"metadata,omitempty"`
}
