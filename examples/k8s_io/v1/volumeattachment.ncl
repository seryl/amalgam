# Module: k8s.io.v1.volumeattachment

let volumeattachmentspec = import "./volumeattachmentspec.ncl" in
let objectmeta = import "./objectmeta.ncl" in
let volumeattachmentstatus = import "./volumeattachmentstatus.ncl" in

{
  # VolumeAttachment captures the intent to attach or detach the specified volume to/from the specified node.
  # 
  # VolumeAttachment objects are non-namespaced.
  VolumeAttachment = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    spec | volumeattachmentspec.VolumeAttachmentSpec | doc m%"
spec represents specification of the desired attach/detach volume behavior. Populated by the Kubernetes system.
"%,
    status | optional | volumeattachmentstatus.VolumeAttachmentStatus | doc m%"
status represents status of the VolumeAttachment request. Populated by the entity completing the attach or detach operation, i.e. the external-attacher.
"%,
  },
}
