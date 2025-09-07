let volumeAttachmentSource = import "./VolumeAttachmentSource.ncl" in

# Module: k8s.io.v1

# VolumeAttachmentSpec is the specification of a VolumeAttachment request.
{
  attacher | String | doc m%"
attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
"%,
  nodeName | String | doc "nodeName represents the node that the volume should be attached to.",
  source | volumeAttachmentSource | doc "source represents the volume that should be attached."
}