# Module: k8s.io.v1.typedobjectreference

{
  TypedObjectReference = {
    apiGroup | optional | String | doc m%"
APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
"%,
    kind | String | doc "Kind is the type of resource being referenced",
    name | String | doc "Name is the name of resource being referenced",
    namespace | optional | String | doc m%"
Namespace is the namespace of resource being referenced Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. (Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%,
  },
}
