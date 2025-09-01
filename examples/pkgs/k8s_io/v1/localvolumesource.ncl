# Module: k8s.io.v1.localvolumesource

{
  # Local represents directly-attached storage with node affinity
  LocalVolumeSource = {
    fsType | optional | String | doc m%"
fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a filesystem if unspecified.
"%,
    path | String | doc m%"
path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
"%,
  },
}
