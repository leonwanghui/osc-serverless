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

import (
	"time"
)

// StorageResourceSpec - Detailed info that indicates the type of backend cloud storage services.
type StorageResourceSpec struct {

	Id string `json:"id,omitempty"`

	Name string `json:"name,omitempty"`

	Description string `json:"description,omitempty"`

	CreatedAt time.Time `json:"created_at,omitempty"`

	UpdatedAt time.Time `json:"updated_at,omitempty"`

	CloudProvider CloudProviderInfo `json:"cloud_provider,omitempty"`

	Status string `json:"status,omitempty"`

	AvailabilityZone string `json:"availability_zone,omitempty"`

	Size int32 `json:"size,omitempty"`

	BlockVolumeFragment BlockVolumeResourceFragment `json:"block_volume_fragment,omitempty"`

	FileShareFragment FileShareResourceFragment `json:"file_share_fragment,omitempty"`

	BackupFragment BackupResourceFragment `json:"backup_fragment,omitempty"`
}