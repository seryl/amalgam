# Module: k8s.io.v1.subject

{
  # Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
  Subject = {
    apiGroup | optional | String | doc m%"
APIGroup holds the API group of the referenced subject. Defaults to "" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
"%,
    kind | String | doc m%"
Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
"%,
    name | String | doc "Name of the object being referenced.",
    namespace | optional | String | doc m%"
Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
"%,
  },
}
