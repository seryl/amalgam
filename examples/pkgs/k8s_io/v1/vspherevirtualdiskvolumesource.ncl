# Module: k8s.io.v1.vspherevirtualdiskvolumesource

{
  # Represents a vSphere volume resource.
  VsphereVirtualDiskVolumeSource = {
    fsType | optional | String | doc m%"
fsType is filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
    storagePolicyID | optional | String | doc m%"
storagePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
"%,
    storagePolicyName | optional | String | doc "storagePolicyName is the storage Policy Based Management (SPBM) profile name.",
    volumePath | optional | String | doc "volumePath is the path that identifies vSphere volume vmdk",
  },
}
