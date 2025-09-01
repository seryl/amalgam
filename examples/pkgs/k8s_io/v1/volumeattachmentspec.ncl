# Module: k8s.io.v1.volumeattachmentspec

let volumeattachmentsource = import "./volumeattachmentsource.ncl" in

{
  # VolumeAttachmentSpec is the specification of a VolumeAttachment request.
  VolumeAttachmentSpec = {
    attacher | optional | String | doc m%"
attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
"%,
    nodeName | optional | String | doc "nodeName represents the node that the volume should be attached to.",
    source | optional | volumeattachmentsource.VolumeAttachmentSource | doc "source represents the volume that should be attached.",
  },
}
