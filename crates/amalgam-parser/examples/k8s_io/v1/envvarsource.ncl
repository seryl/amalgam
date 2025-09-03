# EnvVarSource represents a source for the value of an EnvVar.
{
  configMapKeyRef | optional | ConfigMapKeySelector | doc "Selects a key of a ConfigMap.",
  fieldRef | optional | ObjectFieldSelector | doc m%"
Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
"%,
  resourceFieldRef | optional | ResourceFieldSelector | doc m%"
Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
"%,
  secretKeyRef | optional | SecretKeySelector | doc "Selects a key of a secret in the pod's namespace",
}
