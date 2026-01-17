let metav1 = import "../../k8s_io/apimachinery.pkg.apis/meta/v1/mod.ncl" in

# Module: pkg.crossplane.io.v1

{
  apiVersion
    | String
    | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
    | optional,
  kind
    | String
    | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
    | optional,
  metadata
    | metav1.ObjectMeta
    | optional,
  spec
    | {
    commonLabels
      | { _ : String }
      | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. May match selectors of replication controllers
and services.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%
      | optional,
    desiredState
      | String
      | doc "DesiredState of the PackageRevision. Can be either Active or Inactive.",
    ignoreCrossplaneConstraints
      | Bool
      | doc m%"
IgnoreCrossplaneConstraints indicates to the package manager whether to
honor Crossplane version constrains specified by the package.
Default is false.
"%
      = false,
    image
      | String
      | doc "Package image used by install Pod to extract package contents.",
    packagePullPolicy
      | String
      | doc m%"
PackagePullPolicy defines the pull policy for the package. It is also
applied to any images pulled for the package, such as a provider's
controller image.
Default is IfNotPresent.
"%
      = "IfNotPresent",
    packagePullSecrets
      | Array {
      name
        | String
        | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
        = ""
    }
      | doc m%"
PackagePullSecrets are named secrets in the same namespace that can be
used to fetch packages from private registries. They are also applied to
any images pulled for the package, such as a provider's controller image.
"%
      | optional,
    revision
      | Number
      | doc m%"
Revision number. Indicates when the revision will be garbage collected
based on the parent's RevisionHistoryLimit.
"%,
    skipDependencyResolution
      | Bool
      | doc m%"
SkipDependencyResolution indicates to the package manager whether to skip
resolving dependencies for a package. Setting this value to true may have
unintended consequences.
Default is false.
"%
      = false
  }
    | doc "PackageRevisionSpec specifies the desired state of a PackageRevision."
    | optional,
  status
    | {
    appliedImageConfigRefs
      | Array {
      name
        | String
        | doc "Name is the name of the image config.",
      reason
        | String
        | doc "Reason indicates what the image config was used for."
    }
      | doc m%"
AppliedImageConfigRefs records any image configs that were applied in
reconciling this revision, and what they were used for.
"%
      | optional,
    capabilities
      | Array String
      | doc m%"
Capabilities of this package. Capabilities are opaque strings that
may be meaningful to package consumers.
"%
      | optional,
    conditions
      | Array {
      lastTransitionTime
        | String
        | std.contract.from_predicate (fun value => std.string.is_match "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}" value)
        | doc m%"
LastTransitionTime is the last time this condition transitioned from one
status to another.
"%,
      message
        | String
        | doc m%"
A Message containing details about this condition's last transition from
one status to another, if any.
"%
        | optional,
      observedGeneration
        | Number
        | doc m%"
ObservedGeneration represents the .metadata.generation that the condition was set based upon.
For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date
with respect to the current state of the instance.
"%
        | optional,
      reason
        | String
        | doc "A Reason for this condition's last transition from one status to another.",
      status
        | String
        | doc "Status of this condition; is it currently True, False, or Unknown?",
      "type"
        | String
        | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%
    }
      | doc "Conditions of the resource."
      | optional,
    foundDependencies
      | Number
      | doc "Dependency information."
      | optional,
    installedDependencies
      | Number
      | optional,
    invalidDependencies
      | Number
      | optional,
    objectRefs
      | Array {
      apiVersion
        | String
        | doc "APIVersion of the referenced object.",
      kind
        | String
        | doc "Kind of the referenced object.",
      name
        | String
        | doc "Name of the referenced object.",
      uid
        | String
        | doc "UID of the referenced object."
        | optional
    }
      | doc "References to objects owned by PackageRevision."
      | optional,
    resolvedImage
      | String
      | doc m%"
ResolvedPackage is the name of the package that was installed. It may be
different from spec.image if the package path was rewritten using an
image config.
"%
      | optional
  }
    | doc "PackageRevisionStatus represents the observed state of a PackageRevision."
    | optional
}