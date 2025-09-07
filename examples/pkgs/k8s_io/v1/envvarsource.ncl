let configMapKeySelector = import "./ConfigMapKeySelector.ncl" in
let objectFieldSelector = import "./ObjectFieldSelector.ncl" in
let resourceFieldSelector = import "./ResourceFieldSelector.ncl" in
let secretKeySelector = import "./SecretKeySelector.ncl" in

# Module: k8s.io.v1

# EnvVarSource represents a source for the value of an EnvVar.
{
  configMapKeyRef
     | configMapKeySelector
     | doc "Selects a key of a ConfigMap."
     | optional,
  fieldRef
     | objectFieldSelector
     | doc m%"
Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
"%
     | optional,
  resourceFieldRef
     | resourceFieldSelector
     | doc m%"
Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
"%
     | optional,
  secretKeyRef
     | secretKeySelector
     | doc "Selects a key of a secret in the pod's namespace"
     | optional
}