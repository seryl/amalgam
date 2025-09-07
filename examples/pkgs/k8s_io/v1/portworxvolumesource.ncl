# Module: k8s.io.v1

# PortworxVolumeSource represents a Portworx volume resource.
{
  fsType
     | String
     | doc m%"
fSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
"%
     | optional,
  readOnly
     | Bool
     | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
     | optional,
  volumeID
     | String
     | doc "volumeID uniquely identifies a Portworx volume"
}