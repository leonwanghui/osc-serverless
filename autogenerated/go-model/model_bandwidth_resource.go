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

type BandwidthResource struct {
	Name string `json:"name,omitempty"`

	Sharetype string `json:"sharetype"`

	Size int32 `json:"size,omitempty"`
}
