# EnvFromSource represents the source of a set of ConfigMaps or Secrets
{
  configMapRef | optional | ConfigMapEnvSource | doc "The ConfigMap to select from",
  prefix | optional | String | doc m%"
Optional text to prepend to the name of each environment variable. Must be a C_IDENTIFIER.
"%,
  secretRef | optional | SecretEnvSource | doc "The Secret to select from",
}
