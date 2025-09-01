# Module: k8s.io.v1.envfromsource

let secretenvsource = import "./secretenvsource.ncl" in
let configmapenvsource = import "./configmapenvsource.ncl" in

{
  # EnvFromSource represents the source of a set of ConfigMaps
  EnvFromSource = {
    configMapRef | optional | configmapenvsource.ConfigMapEnvSource | doc "The ConfigMap to select from",
    prefix | optional | String | doc m%"
An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
"%,
    secretRef | optional | secretenvsource.SecretEnvSource | doc "The Secret to select from",
  },
}
