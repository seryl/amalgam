# Module: k8s.io.v1

# IngressClassParametersReference identifies an API object. This can be used to specify a cluster or namespace-scoped resource.
{
  apiGroup | optional | String | doc m%"
apiGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
"%,
  kind | optional | String | doc "kind is the type of resource being referenced.",
  name | optional | String | doc "name is the name of resource being referenced.",
  namespace | optional | String | doc m%"
namespace is the namespace of the resource being referenced. This field is required when scope is set to "Namespace" and must be unset when scope is set to "Cluster".
"%,
  scope | optional | String | doc m%"
scope represents if this refers to a cluster or namespace scoped resource. This may be set to "Cluster" (default) or "Namespace".
"%
}