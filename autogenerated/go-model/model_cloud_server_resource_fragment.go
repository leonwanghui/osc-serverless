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

type CloudServerResourceFragment struct {
	HostId string `json:"hostId,omitempty"`

	KeyName string `json:"key_name,omitempty"`

	OSEXTSTSvmState string `json:"OS-EXT-STS:vm_state,omitempty"`

	Addresses map[string]interface{} `json:"addresses,omitempty"`

	Flavor CloudServerResourceFragmentFlavor `json:"flavor,omitempty"`

	OSEXTAZavailabilityZone string `json:"OS-EXT-AZ:availability_zone,omitempty"`

	AccessIPv4 string `json:"accessIPv4,omitempty"`

	Metadata map[string]interface{} `json:"metadata,omitempty"`

	OsExtendedVolumesvolumesAttached []CloudServerResourceFragmentOsextendedvolumesvolumesAttached `json:"os-extended-volumes:volumes_attached,omitempty"`

	OSEXTSRVATTRhostname string `json:"OS-EXT-SRV-ATTR:hostname,omitempty"`
}
