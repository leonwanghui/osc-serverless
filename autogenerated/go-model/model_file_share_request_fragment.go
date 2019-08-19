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

type FileShareRequestFragment struct {

	ShareProto string `json:"share_proto"`

	ShareType string `json:"share_type,omitempty"`

	Metadata map[string]interface{} `json:"metadata,omitempty"`

	ShareNetworkId string `json:"share_network_id,omitempty"`
}