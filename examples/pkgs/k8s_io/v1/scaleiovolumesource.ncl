let localobjectreference = import "./localobjectreference.ncl" in

# Module: k8s.io.v1

# ScaleIOVolumeSource represents a persistent ScaleIO volume
{
  fsType | optional | String | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
"%,
  gateway | optional | String | doc "gateway is the host address of the ScaleIO API Gateway.",
  protectionDomain | optional | String | doc m%"
protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
"%,
  readOnly | optional | Bool | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%,
  secretRef | optional | localobjectreference | doc m%"
secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
"%,
  sslEnabled | optional | Bool | doc "sslEnabled Flag enable/disable SSL communication with Gateway, default false",
  storageMode | optional | String | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
"%,
  storagePool | optional | String | doc "storagePool is the ScaleIO Storage Pool associated with the protection domain.",
  system | optional | String | doc "system is the name of the storage system as configured in ScaleIO.",
  volumeName | optional | String | doc m%"
volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
"%
}