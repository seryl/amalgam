# Module: lock.pkg.crossplane.io

let k8s_io_v1 = import "../../k8s_io/v1/objectmeta.ncl" in

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
    metadata | optional | k8s_io_v1.ObjectMeta,
    packages | optional | Array {
      apiVersion | optional | String | doc "APIVersion of the package.",
      dependencies | Array {
        apiVersion | optional | String | doc "APIVersion of the package.",
        constraints | String | doc m%"
Constraints is a valid semver range or a digest, which will be used to select a valid
dependency version.
"%,
        kind | optional | String | doc "Kind of the package (not the kind of the package revision).",
        package | String | doc "Package is the OCI image name without a tag or digest.",
        type | optional | String | doc m%"
Type is the type of package. Can be either Configuration or Provider.
Deprecated: Specify an apiVersion and kind instead.
"%,
      } | doc m%"
Dependencies are the list of dependencies of this package. The order of
the dependencies will dictate the order in which they are resolved.
"%,
      kind | optional | String | doc "Kind of the package (not the kind of the package revision).",
      name | String | doc "Name corresponds to the name of the package revision for this package.",
      source | String | doc "Source is the OCI image name without a tag or digest.",
      type | optional | String | doc m%"
Type is the type of package.
Deprecated: Specify an apiVersion and kind instead.
"%,
      version | String | doc "Version is the tag or digest of the OCI image.",
    },
    status | optional | {
      conditions | optional | Array {
        lastTransitionTime | String | doc m%"
LastTransitionTime is the last time this condition transitioned from one
status to another.
"%,
        message | optional | String | doc m%"
A Message containing details about this condition's last transition from
one status to another, if any.
"%,
        observedGeneration | optional | Number | doc m%"
ObservedGeneration represents the .metadata.generation that the condition was set based upon.
For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date
with respect to the current state of the instance.
"%,
        reason | String | doc "A Reason for this condition's last transition from one status to another.",
        status | String | doc "Status of this condition; is it currently True, False, or Unknown?",
        type | String | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%,
      } | doc "Conditions of the resource.",
    } | doc "Status of the Lock.",
  },
}
