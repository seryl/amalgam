# Module: k8s.io.v1beta1.servicecidrlist

let servicecidr = import "./servicecidr.ncl" in
let ListMeta = import "../v1/listmeta.ncl" in

{
  # ServiceCIDRList contains a list of ServiceCIDR objects.
  ServiceCIDRList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array servicecidr.ServiceCIDR | doc "items is the list of ServiceCIDRs.",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | ListMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  },
}
