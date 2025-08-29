# Module: k8s.io.v1.controllerrevisionlist

let controllerrevision = import "./controllerrevision.ncl" in
let listmeta = import "./listmeta.ncl" in

{
  # ControllerRevisionList is a resource containing a list of ControllerRevision objects.
  ControllerRevisionList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array controllerrevision.ControllerRevision | doc "Items is the list of ControllerRevisions",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc m%"
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  },
}
