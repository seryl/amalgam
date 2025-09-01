# Module: configuration.pkg.crossplane.io

let k8s_io_objectmeta = import "github:seryl/nickel-pkgs/k8s_io" in

{
  Configuration = {
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
    spec | optional | {
      commonLabels | optional | {
      } | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. May match selectors of replication controllers
and services.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%,
      ignoreCrossplaneConstraints | Bool | doc m%"
IgnoreCrossplaneConstraints indicates to the package manager whether to
honor Crossplane version constrains specified by the package.
Default is false.
"% | default = false,
      package | optional | String | doc m%"
Package is the name of the package that is being requested.
must be a fully qualified image name, including the registry,
repository, and tag. for example, "registry.example.com/repo/package:tag".
"%,
      packagePullPolicy | String | doc m%"
PackagePullPolicy defines the pull policy for the package.
Default is IfNotPresent.
"% | default = "IfNotPresent",
      packagePullSecrets | optional | Array {
        name | String | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"% | default = "",
      } | doc m%"
PackagePullSecrets are named secrets in the same namespace that can be used
to fetch packages from private registries.
"%,
      revisionActivationPolicy | String | doc m%"
RevisionActivationPolicy specifies how the package controller should
update from one revision to the next. Options are Automatic or Manual.
Default is Automatic.
"% | default = "Automatic",
      revisionHistoryLimit | Number | doc m%"
RevisionHistoryLimit dictates how the package controller cleans up old
inactive package revisions.
Defaults to 1. Can be disabled by explicitly setting to 0.
"% | default = 1,
      skipDependencyResolution | Bool | doc m%"
SkipDependencyResolution indicates to the package manager whether to skip
resolving dependencies for a package. Setting this value to true may have
unintended consequences.
Default is false.
"% | default = false,
    } | doc m%"
ConfigurationSpec specifies details about a request to install a
configuration to Crossplane.
"%,
    status | optional | {
      appliedImageConfigRefs | optional | Array {
        name | optional | String | doc "Name is the name of the image config.",
        reason | optional | String | doc "Reason indicates what the image config was used for.",
      } | doc m%"
AppliedImageConfigRefs records any image configs that were applied in
reconciling this package, and what they were used for.
"%,
      conditions | optional | Array {
        lastTransitionTime | optional | String | doc m%"
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
        reason | optional | String | doc "A Reason for this condition's last transition from one status to another.",
        status | optional | String | doc "Status of this condition; is it currently True, False, or Unknown?",
        type | optional | String | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%,
      } | doc "Conditions of the resource.",
      currentIdentifier | optional | String | doc m%"
CurrentIdentifier is the most recent package source that was used to
produce a revision. The package manager uses this field to determine
whether to check for package updates for a given source when
packagePullPolicy is set to IfNotPresent. Manually removing this field
will cause the package manager to check that the current revision is
correct for the given package source.
"%,
      currentRevision | optional | String | doc m%"
CurrentRevision is the name of the current package revision. It will
reflect the most up to date revision, whether it has been activated or
not.
"%,
      resolvedPackage | optional | String | doc m%"
ResolvedPackage is the name of the package that was used for version
resolution. It may be different from spec.package if the package path was
rewritten using an image config.
"%,
    } | doc "ConfigurationStatus represents the observed state of a Configuration.",
  },
}
