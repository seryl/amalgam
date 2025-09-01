# Module: k8s.io.v1.clusterrolebinding

let subject = import "./subject.ncl" in
let roleref = import "./roleref.ncl" in
let objectmeta = import "./objectmeta.ncl" in

{
  # ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.
  ClusterRoleBinding = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc "Standard object's metadata.",
    roleRef | optional | roleref.RoleRef | doc m%"
RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error. This field is immutable.
"%,
    subjects | optional | Array subject.Subject | doc "Subjects holds references to the objects the role applies to.",
  },
}
