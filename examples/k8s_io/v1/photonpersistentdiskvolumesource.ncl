# Module: k8s.io.v1.photonpersistentdiskvolumesource

{
  # Represents a Photon Controller persistent disk resource.
  PhotonPersistentDiskVolumeSource = {
    fsType | optional | String | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
    pdID | String | doc "pdID is the ID that identifies Photon Controller persistent disk",
  },
}
