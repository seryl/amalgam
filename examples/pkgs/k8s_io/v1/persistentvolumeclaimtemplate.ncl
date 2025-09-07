let objectMeta = import "./ObjectMeta.ncl" in
let persistentVolumeClaimSpec = import "./PersistentVolumeClaimSpec.ncl" in

# Module: k8s.io.v1

# PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects as part of an EphemeralVolumeSource.
{
  metadata | optional | objectMeta | doc m%"
May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
"%,
  spec | persistentVolumeClaimSpec | doc m%"
The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
"%
}