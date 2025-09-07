let objectMeta = import "./ObjectMeta.ncl" in
let statefulSetSpec = import "./StatefulSetSpec.ncl" in
let statefulSetStatus = import "./StatefulSetStatus.ncl" in

# Module: k8s.io.v1

# StatefulSet represents a set of pods with consistent identities. Identities are defined as:
#   - Network: A single stable DNS and hostname.
#   - Storage: As many VolumeClaims as requested.
# 
# The StatefulSet guarantees that a given network identity will always map to the same storage identity.
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
     | statefulSetSpec
     | doc "Spec defines the desired identities of pods in this set."
     | optional,
  status
     | statefulSetStatus
     | doc m%"
Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
"%
     | optional
}