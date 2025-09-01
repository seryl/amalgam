# Module: configurationrevision.pkg.crossplane.io

let k8s_io_objectmeta = import "k8s_io" in

{
  ConfigurationRevision = {
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
      desiredState | String | doc "DesiredState of the PackageRevision. Can be either Active or Inactive.",
      ignoreCrossplaneConstraints | Bool | doc m%"
IgnoreCrossplaneConstraints indicates to the package manager whether to
honor Crossplane version constrains specified by the package.
Default is false.
"% | default = false,
      image | String | doc "Package image used by install Pod to extract package contents.",
      packagePullPolicy | String | doc m%"
PackagePullPolicy defines the pull policy for the package. It is also
applied to any images pulled for the package, such as a provider's
controller image.
Default is IfNotPresent.
"% | default = "IfNotPresent",
      packagePullSecrets | optional | Array {
        name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
      } | doc m%"
PackagePullSecrets are named secrets in the same namespace that can be
used to fetch packages from private registries. They are also applied to
any images pulled for the package, such as a provider's controller image.
"%,
      revision | Number | doc m%"
Revision number. Indicates when the revision will be garbage collected
based on the parent's RevisionHistoryLimit.
"%,
      skipDependencyResolution | Bool | doc m%"
SkipDependencyResolution indicates to the package manager whether to skip
resolving dependencies for a package. Setting this value to true may have
unintended consequences.
Default is false.
"% | default = false,
    } | doc "PackageRevisionSpec specifies the desired state of a PackageRevision.",
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
      foundDependencies | optional | Number | doc "Dependency information.",
      installedDependencies | optional | Number,
      invalidDependencies | optional | Number,
      objectRefs | optional | Array {
        apiVersion | String | doc "APIVersion of the referenced object.",
        kind | String | doc "Kind of the referenced object.",
        name | String | doc "Name of the referenced object.",
        uid | optional | String | doc "UID of the referenced object.",
      } | doc "References to objects owned by PackageRevision.",
      permissionRequests | optional | Array {
        apiGroups | optional | Array String | doc m%"
APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of
the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
"%,
        nonResourceURLs | optional | Array String | doc m%"
NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path
Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding.
Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
"%,
        resourceNames | optional | Array String | doc m%"
ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
"%,
        resources | optional | Array String | doc m%"
Resources is a list of resources this rule applies to. '*' represents all resources.
"%,
        verbs | Array String | doc m%"
Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
"%,
      } | doc m%"
PermissionRequests made by this package. The package declares that its
controller needs these permissions to run. The RBAC manager is
responsible for granting them.
"%,
    } | doc "PackageRevisionStatus represents the observed state of a PackageRevision.",
  },
}
