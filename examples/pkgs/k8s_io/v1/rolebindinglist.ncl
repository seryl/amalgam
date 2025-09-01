# Module: k8s.io.v1.rolebindinglist

let rolebinding = import "./rolebinding.ncl" in
let listmeta = import "./listmeta.ncl" in

{
  # RoleBindingList is a collection of RoleBindings
  RoleBindingList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array rolebinding.RoleBinding | doc "Items is a list of RoleBindings",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc "Standard object's metadata.",
  },
}
