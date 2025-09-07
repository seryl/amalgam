# Module: k8s.io.v1

# AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
{
  cachingMode
     | String
     | doc "cachingMode is the Host Caching mode: None, Read Only, Read Write."
     | optional,
  diskName
     | String
     | doc "diskName is the Name of the data disk in the blob storage",
  diskURI
     | String
     | doc "diskURI is the URI of data disk in the blob storage",
  fsType
     | String
     | doc m%"
fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
     | optional,
  kind
     | String
     | doc m%"
kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
"%
     | optional,
  readOnly
     | Bool
     | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
     | optional
}