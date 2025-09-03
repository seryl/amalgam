# TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.
{
  apiGroup | optional | String | doc m%"
APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
"%,
  kind | optional | String | doc "Kind is the type of resource being referenced",
  name | optional | String | doc "Name is the name of resource being referenced",
}
