# Module: k8s.io.v1.ingressclasslist

let ingressclass = import "./ingressclass.ncl" in
let listmeta = import "./listmeta.ncl" in

{
  # IngressClassList is a collection of IngressClasses.
  IngressClassList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array ingressclass.IngressClass | doc "items is the list of IngressClasses.",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc "Standard list metadata.",
  },
}
