# Module: k8s.io.v1.nodeconfigsource

let configmapnodeconfigsource = import "./configmapnodeconfigsource.ncl" in

{
  # NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22
  NodeConfigSource = {
    configMap | optional | configmapnodeconfigsource.ConfigMapNodeConfigSource | doc "ConfigMap is a reference to a Node's ConfigMap",
  },
}
