let objectmeta = import "./objectmeta.ncl" in
let statefulsetspec = import "./statefulsetspec.ncl" in
let statefulsetstatus = import "./statefulsetstatus.ncl" in

# Module: k8s.io.v1

# StatefulSet represents a set of pods with consistent identities. Identities are defined as:
#   - Network: A single stable DNS and hostname.
#   - Storage: As many VolumeClaims as requested.
# 
# The StatefulSet guarantees that a given network identity will always map to the same storage identity.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectmeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | statefulsetspec | doc "Spec defines the desired identities of pods in this set.",
  status | optional | statefulsetstatus | doc m%"
Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
"%
}