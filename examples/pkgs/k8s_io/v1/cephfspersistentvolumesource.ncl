# Module: k8s.io.v1.cephfspersistentvolumesource

let secretreference = import "./secretreference.ncl" in

{
  # Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
  CephFSPersistentVolumeSource = {
    monitors | optional | Array String | doc m%"
monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
    path | optional | String | doc m%"
path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
"%,
    readOnly | optional | Bool | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
    secretFile | optional | String | doc m%"
secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
    secretRef | optional | secretreference.SecretReference | doc m%"
secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
    user | optional | String | doc m%"
user is Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
  },
}
