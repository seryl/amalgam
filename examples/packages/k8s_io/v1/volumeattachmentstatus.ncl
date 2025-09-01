# Module: k8s.io.v1.volumeattachmentstatus

let volumeerror = import "./volumeerror.ncl" in

{
  # VolumeAttachmentStatus is the status of a VolumeAttachment request.
  VolumeAttachmentStatus = {
    attachError | optional | volumeerror.VolumeError | doc m%"
attachError represents the last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
"%,
    attached | Bool | doc m%"
attached indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
"%,
    attachmentMetadata | optional | { .. } | doc m%"
attachmentMetadata is populated with any information returned by the attach operation, upon successful attach, that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
"%,
    detachError | optional | volumeerror.VolumeError | doc m%"
detachError represents the last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.
"%,
  },
}
