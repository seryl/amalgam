# Module: k8s.io.v1

# VolumeMountStatus shows status of volume mounts.
{
  mountPath | optional | String | doc "MountPath corresponds to the original VolumeMount.",
  name | optional | String | doc "Name corresponds to the name of the original VolumeMount.",
  readOnly | optional | Bool | doc "ReadOnly corresponds to the original VolumeMount.",
  recursiveReadOnly | optional | String | doc m%"
RecursiveReadOnly must be set to Disabled, Enabled, or unspecified (for non-readonly mounts). An IfPossible value in the original VolumeMount must be translated to Disabled or Enabled, depending on the mount result.
"%
}