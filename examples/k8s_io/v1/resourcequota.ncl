# Module: k8s.io.v1.resourcequota

let objectmeta = import "./objectmeta.ncl" in
let resourcequotastatus = import "./resourcequotastatus.ncl" in
let resourcequotaspec = import "./resourcequotaspec.ncl" in

{
  # ResourceQuota sets aggregate quota restrictions enforced per namespace
  ResourceQuota = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    spec | optional | resourcequotaspec.ResourceQuotaSpec | doc m%"
Spec defines the desired quota. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
    status | optional | resourcequotastatus.ResourceQuotaStatus | doc m%"
Status defines the actual enforced quota and its current usage. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
  },
}
