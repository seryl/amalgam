# Module: k8s.io.v1

# ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration
{
  kubeletConfigKey | optional | String | doc m%"
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
"%,
  name | optional | String | doc m%"
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
"%,
  namespace | optional | String | doc m%"
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
"%,
  resourceVersion | optional | String | doc m%"
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
"%,
  uid | optional | String | doc m%"
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
"%
}
