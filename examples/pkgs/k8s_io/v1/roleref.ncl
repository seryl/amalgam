# Module: k8s.io.v1

# RoleRef contains information that points to the role being used
{
  apiGroup | optional | String | doc "APIGroup is the group for the resource being referenced",
  kind | optional | String | doc "Kind is the type of resource being referenced",
  name | optional | String | doc "Name is the name of resource being referenced"
}