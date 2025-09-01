# Module: k8s.io.v1.persistentvolumeclaimvolumesource

{
  # PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).
  PersistentVolumeClaimVolumeSource = {
    claimName | optional | String | doc m%"
claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%,
    readOnly | optional | Bool | doc "readOnly Will force the ReadOnly setting in VolumeMounts. Default false.",
  },
}
