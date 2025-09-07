let localObjectReference = import "./LocalObjectReference.ncl" in

# Module: k8s.io.v1

# ScaleIOVolumeSource represents a persistent ScaleIO volume
{
  fsType
     | String
     | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
"%
     | optional,
  gateway
     | String
     | doc "gateway is the host address of the ScaleIO API Gateway.",
  protectionDomain
     | String
     | doc m%"
protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
"%
     | optional,
  readOnly
     | Bool
     | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
     | optional,
  secretRef
     | localObjectReference
     | doc m%"
secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
"%,
  sslEnabled
     | Bool
     | doc "sslEnabled Flag enable/disable SSL communication with Gateway, default false"
     | optional,
  storageMode
     | String
     | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
"%
     | optional,
  storagePool
     | String
     | doc "storagePool is the ScaleIO Storage Pool associated with the protection domain."
     | optional,
  system
     | String
     | doc "system is the name of the storage system as configured in ScaleIO.",
  volumeName
     | String
     | doc m%"
volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
"%
     | optional
}