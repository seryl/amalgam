# Module: k8s.io.v1

# Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.
{
  fsType | optional | String | doc m%"
fsType Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
  readOnly | optional | Bool | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
  secretRef | optional | secretreference.SecretReference | doc m%"
secretRef is Optional: points to a secret object containing parameters used to connect to OpenStack.
"%,
  volumeID | optional | String | doc m%"
volumeID used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
}
