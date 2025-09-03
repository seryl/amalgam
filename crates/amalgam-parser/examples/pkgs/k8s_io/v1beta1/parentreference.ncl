# Module: k8s.io.v1beta1

# ParentReference describes a reference to a parent object.
{
  group | optional | String | doc "Group is the group of the object being referenced.",
  name | optional | String | doc "Name is the name of the object being referenced.",
  namespace | optional | String | doc "Namespace is the namespace of the object being referenced.",
  resource | optional | String | doc "Resource is the resource of the object being referenced."
}
