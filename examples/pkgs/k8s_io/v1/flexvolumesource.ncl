let localObjectReference = import "./LocalObjectReference.ncl" in

# Module: k8s.io.v1

# FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
{
  driver
     | String
     | doc "driver is the name of the driver to use for this volume.",
  fsType
     | String
     | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%
     | optional,
  options
     | { .. }
     | doc "options is Optional: this field holds extra command options if any."
     | optional,
  readOnly
     | Bool
     | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
     | optional,
  secretRef
     | localObjectReference
     | doc m%"
secretRef is Optional: secretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
"%
     | optional
}