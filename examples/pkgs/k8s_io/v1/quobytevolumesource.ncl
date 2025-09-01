# Module: k8s.io.v1.quobytevolumesource

{
  # Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
  QuobyteVolumeSource = {
    group | optional | String | doc "group to map volume access to Default is no group",
    readOnly | optional | Bool | doc m%"
readOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
"%,
    registry | optional | String | doc m%"
registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
"%,
    tenant | optional | String | doc m%"
tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
"%,
    user | optional | String | doc "user to map volume access to Defaults to serivceaccount user",
    volume | optional | String | doc "volume is a string that references an already created Quobyte volume by name.",
  },
}
