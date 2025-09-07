# Module: k8s.io.v1

# Represents a vSphere volume resource.
{
  fsType
     | String
     | doc m%"
fsType is filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
     | optional,
  storagePolicyID
     | String
     | doc m%"
storagePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
"%
     | optional,
  storagePolicyName
     | String
     | doc "storagePolicyName is the storage Policy Based Management (SPBM) profile name."
     | optional,
  volumePath
     | String
     | doc "volumePath is the path that identifies vSphere volume vmdk"
}