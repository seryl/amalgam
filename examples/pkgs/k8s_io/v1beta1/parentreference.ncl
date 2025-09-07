# Module: k8s.io.v1beta1

# ParentReference describes a reference to a parent object.
{
  group
     | String
     | doc "Group is the group of the object being referenced."
     | optional,
  name
     | String
     | doc "Name is the name of the object being referenced.",
  namespace
     | String
     | doc "Namespace is the namespace of the object being referenced."
     | optional,
  resource
     | String
     | doc "Resource is the resource of the object being referenced."
}