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

type NetworkResourceCreateRequest struct {
	CloudProvider CloudProviderInfo `json:"cloud_provider"`

	Name string `json:"name"`

	Description string `json:"description,omitempty"`

	VpcRequestFragment SubnetRequestFragment `json:"vpc_request_fragment,omitempty"`

	SubnetRequestFragment SubnetRequestFragment `json:"subnet_request_fragment,omitempty"`

	PublicipRequestFragment PublicipRequestFragment `json:"publicip_request_fragment,omitempty"`
}
