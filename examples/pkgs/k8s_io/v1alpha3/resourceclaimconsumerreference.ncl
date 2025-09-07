# Module: k8s.io.v1alpha3

# ResourceClaimConsumerReference contains enough information to let you locate the consumer of a ResourceClaim. The user must be a resource in the same namespace as the ResourceClaim.
{
  apiGroup
     | String
     | doc m%"
APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.
"%
     | optional,
  name
     | String
     | doc "Name is the name of resource being referenced.",
  resource
     | String
     | doc "Resource is the type of resource being referenced, for example \"pods\".",
  uid
     | String
     | doc "UID identifies exactly one incarnation of the resource."
}