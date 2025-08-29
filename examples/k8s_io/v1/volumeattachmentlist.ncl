# Module: k8s.io.v1.volumeattachmentlist

let volumeattachment = import "./volumeattachment.ncl" in
let listmeta = import "./listmeta.ncl" in

{
  # VolumeAttachmentList is a collection of VolumeAttachment objects.
  VolumeAttachmentList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array volumeattachment.VolumeAttachment | doc "items is the list of VolumeAttachments",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc m%"
Standard list metadata More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  },
}
