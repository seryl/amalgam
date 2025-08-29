# Module: k8s.io.v1.storageosvolumesource

let localobjectreference = import "./localobjectreference.ncl" in

{
  # Represents a StorageOS persistent volume resource.
  StorageOSVolumeSource = {
    fsType | optional | String | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
    readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
    secretRef | optional | localobjectreference.LocalObjectReference | doc m%"
secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
"%,
    volumeName | optional | String | doc m%"
volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
"%,
    volumeNamespace | optional | String | doc m%"
volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
"%,
  },
}
