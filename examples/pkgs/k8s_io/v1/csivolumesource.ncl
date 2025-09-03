let localobjectreference = import "./localobjectreference.ncl" in

# Module: k8s.io.v1

# Represents a source location of a volume to mount, managed by an external CSI driver
{
  driver | optional | String | doc m%"
driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
"%,
  fsType | optional | String | doc m%"
fsType to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
"%,
  nodePublishSecretRef | optional | localobjectreference | doc m%"
nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
"%,
  readOnly | optional | Bool | doc m%"
readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).
"%,
  volumeAttributes | optional | { .. } | doc m%"
volumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
"%
}