# Module: k8s.io.v1.azurediskvolumesource

{
  # AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
  AzureDiskVolumeSource = {
    cachingMode | optional | String | doc "cachingMode is the Host Caching mode: None, Read Only, Read Write.",
    diskName | String | doc "diskName is the Name of the data disk in the blob storage",
    diskURI | String | doc "diskURI is the URI of data disk in the blob storage",
    fsType | optional | String | doc m%"
fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
    kind | optional | String | doc m%"
kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
"%,
    readOnly | optional | Bool | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
  },
}
