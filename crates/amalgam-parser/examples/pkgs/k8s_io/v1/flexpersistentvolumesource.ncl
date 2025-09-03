# Module: k8s.io.v1

# FlexPersistentVolumeSource represents a generic persistent volume resource that is provisioned/attached using an exec based plugin.
{
  driver | optional | String | doc "driver is the name of the driver to use for this volume.",
  fsType | optional | String | doc m%"
fsType is the Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%,
  options | optional | { .. } | doc "options is Optional: this field holds extra command options if any.",
  readOnly | optional | Bool | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
  secretRef | optional | secretreference.SecretReference | doc m%"
secretRef is Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
"%
}
