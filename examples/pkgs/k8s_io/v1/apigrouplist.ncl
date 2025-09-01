# Module: k8s.io.v1.apigrouplist

let apigroup = import "./apigroup.ncl" in

{
  # APIGroupList is a list of APIGroup, to allow clients to discover the API at /apis.
  APIGroupList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    groups | Array apigroup.APIGroup | doc "groups is a list of APIGroup.",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  },
}
