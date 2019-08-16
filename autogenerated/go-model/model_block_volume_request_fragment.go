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

type BlockVolumeRequestFragment struct {

	ImageRef string `json:"imageRef,omitempty"`

	VolumeType string `json:"volume_type"`

	Count int32 `json:"count,omitempty"`

	Sharable string `json:"sharable,omitempty"`

	Metadata map[string]interface{} `json:"metadata,omitempty"`

	Multiattach bool `json:"multiattach,omitempty"`
}
