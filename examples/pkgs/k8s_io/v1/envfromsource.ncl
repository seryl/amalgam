let configMapEnvSource = import "./ConfigMapEnvSource.ncl" in
let secretEnvSource = import "./SecretEnvSource.ncl" in

# Module: k8s.io.v1

# EnvFromSource represents the source of a set of ConfigMaps or Secrets
{
  configMapRef | optional | configMapEnvSource | doc "The ConfigMap to select from",
  prefix | optional | String | doc m%"
Optional text to prepend to the name of each environment variable. Must be a C_IDENTIFIER.
"%,
  secretRef | optional | secretEnvSource | doc "The Secret to select from"
}