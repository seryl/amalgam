# Module: k8s.io.v1alpha3

# ResourceClaimConsumerReference contains enough information to let you locate the consumer of a ResourceClaim. The user must be a resource in the same namespace as the ResourceClaim.
{
  apiGroup | optional | String | doc m%"
APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.
"%,
  name | optional | String | doc "Name is the name of resource being referenced.",
  resource | optional | String | doc "Resource is the type of resource being referenced, for example \"pods\".",
  uid | optional | String | doc "UID identifies exactly one incarnation of the resource."
}
