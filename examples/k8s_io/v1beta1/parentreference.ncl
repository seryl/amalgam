# Module: k8s.io.v1beta1.parentreference

{
  # ParentReference describes a reference to a parent object.
  ParentReference = {
    group | optional | String | doc "Group is the group of the object being referenced.",
    name | String | doc "Name is the name of the object being referenced.",
    namespace | optional | String | doc "Namespace is the namespace of the object being referenced.",
    resource | String | doc "Resource is the resource of the object being referenced.",
  },
}
