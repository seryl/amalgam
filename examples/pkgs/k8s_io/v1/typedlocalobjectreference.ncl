# Module: k8s.io.v1

# TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.
{
  apiGroup
     | String
     | doc m%"
APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
"%
     | optional,
  kind
     | String
     | doc "Kind is the type of resource being referenced",
  name
     | String
     | doc "Name is the name of resource being referenced"
}