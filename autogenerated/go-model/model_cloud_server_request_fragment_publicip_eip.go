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

type CloudServerRequestFragmentPublicipEip struct {
	IpType string `json:"ip_type"`

	Bandwidth BandwidthResource `json:"bandwidth"`
}
