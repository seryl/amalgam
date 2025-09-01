# Module: k8s.io.v1.envvarsource

let configmapkeyselector = import "./configmapkeyselector.ncl" in
let resourcefieldselector = import "./resourcefieldselector.ncl" in
let objectfieldselector = import "./objectfieldselector.ncl" in
let secretkeyselector = import "./secretkeyselector.ncl" in

{
  # EnvVarSource represents a source for the value of an EnvVar.
  EnvVarSource = {
    configMapKeyRef | optional | configmapkeyselector.ConfigMapKeySelector | doc "Selects a key of a ConfigMap.",
    fieldRef | optional | objectfieldselector.ObjectFieldSelector | doc m%"
Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
"%,
    resourceFieldRef | optional | resourcefieldselector.ResourceFieldSelector | doc m%"
Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
"%,
    secretKeyRef | optional | secretkeyselector.SecretKeySelector | doc "Selects a key of a secret in the pod's namespace",
  },
}
