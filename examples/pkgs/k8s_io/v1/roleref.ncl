# Module: k8s.io.v1

# RoleRef contains information that points to the role being used
{
  apiGroup | String | doc "APIGroup is the group for the resource being referenced",
  kind | String | doc "Kind is the type of resource being referenced",
  name | String | doc "Name is the name of resource being referenced"
}