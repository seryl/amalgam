# Module: k8s.io.v1.rbdvolumesource

let localobjectreference = import "./localobjectreference.ncl" in

{
  # Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
  RBDVolumeSource = {
    fsType | optional | String | doc m%"
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
"%,
    image | optional | String | doc m%"
image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    keyring | optional | String | doc m%"
keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    monitors | optional | Array String | doc m%"
monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    pool | optional | String | doc m%"
pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    readOnly | optional | Bool | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    secretRef | optional | localobjectreference.LocalObjectReference | doc m%"
secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    user | optional | String | doc m%"
user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
  },
}
