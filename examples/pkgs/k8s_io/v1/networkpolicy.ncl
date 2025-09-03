let objectmeta = import "./objectmeta.ncl" in
let networkpolicyspec = import "./networkpolicyspec.ncl" in

# Module: k8s.io.v1

# NetworkPolicy describes what network traffic is allowed for a set of Pods
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
  spec | optional | networkpolicyspec | doc m%"
spec represents the specification of the desired behavior for this NetworkPolicy.
"%
}