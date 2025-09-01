# Module: k8s.io.v1.limitrange

let objectmeta = import "./objectmeta.ncl" in
let limitrangespec = import "./limitrangespec.ncl" in

{
  # LimitRange sets resource usage limits for each kind of resource in a Namespace.
  LimitRange = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    spec | optional | limitrangespec.LimitRangeSpec | doc m%"
Spec defines the limits enforced. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
  },
}
