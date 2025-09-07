# Module: k8s.io.v1alpha1

# The names of the group, the version, and the resource.
{
  group
     | String
     | doc "The name of the group."
     | optional,
  resource
     | String
     | doc "The name of the resource."
     | optional,
  version
     | String
     | doc "The name of the version."
     | optional
}