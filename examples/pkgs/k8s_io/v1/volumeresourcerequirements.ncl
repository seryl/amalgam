# Module: k8s.io.v1

# VolumeResourceRequirements describes the storage resource requirements for a volume.
{
  limits
     | { .. }
     | doc m%"
Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
     | optional,
  requests
     | { .. }
     | doc m%"
Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
     | optional
}