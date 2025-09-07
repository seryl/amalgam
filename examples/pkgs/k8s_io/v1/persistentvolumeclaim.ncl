let objectMeta = import "./ObjectMeta.ncl" in
let persistentVolumeClaimSpec = import "./PersistentVolumeClaimSpec.ncl" in
let persistentVolumeClaimStatus = import "./PersistentVolumeClaimStatus.ncl" in

# Module: k8s.io.v1

# PersistentVolumeClaim is a user's request for and claim to a persistent volume
{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | objectMeta
     | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
     | optional,
  spec
     | persistentVolumeClaimSpec
     | doc m%"
spec defines the desired characteristics of a volume requested by a pod author. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
     | optional,
  status
     | persistentVolumeClaimStatus
     | doc m%"
status represents the current information/status of a persistent volume claim. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
     | optional
}