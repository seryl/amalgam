# Module: k8s.io.v1

# EnvFromSource represents the source of a set of ConfigMaps or Secrets
{
  configMapRef | optional | configmapenvsource.ConfigMapEnvSource | doc "The ConfigMap to select from",
  prefix | optional | String | doc m%"
Optional text to prepend to the name of each environment variable. Must be a C_IDENTIFIER.
"%,
  secretRef | optional | secretenvsource.SecretEnvSource | doc "The Secret to select from"
}
