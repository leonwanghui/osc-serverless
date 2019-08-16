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

type BackupResourceFragment struct {

	VolumeId string `json:"volume_id,omitempty"`

	Container string `json:"container,omitempty"`

	ServiceMetadata string `json:"service_metadata,omitempty"`

	SnapshotId string `json:"snapshot_id,omitempty"`

	IsIncremental bool `json:"is_incremental,omitempty"`
}
