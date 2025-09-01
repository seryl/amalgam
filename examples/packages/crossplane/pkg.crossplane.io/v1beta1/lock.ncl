# Module: lock.pkg.crossplane.io

let k8s_io_objectmeta = import "k8s_io" in

{
  Lock = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | k8s_io_objectmeta.ObjectMeta,
    packages | optional | Array {
      dependencies | Array {
        constraints | String | doc m%"
Constraints is a valid semver range, which will be used to select a valid
dependency version.
"%,
        package | String | doc "Package is the OCI image name without a tag or digest.",
        type | String | doc "Type is the type of package. Can be either Configuration or Provider.",
      } | doc m%"
Dependencies are the list of dependencies of this package. The order of
the dependencies will dictate the order in which they are resolved.
"%,
      name | String | doc "Name corresponds to the name of the package revision for this package.",
      source | String | doc "Source is the OCI image name without a tag or digest.",
      type | String | doc "Type is the type of package. Can be either Configuration or Provider.",
      version | String | doc "Version is the tag or digest of the OCI image.",
    },
  },
}
