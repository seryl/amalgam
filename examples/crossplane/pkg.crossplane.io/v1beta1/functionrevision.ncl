# Module: functionrevision.pkg.crossplane.io

let k8s_io_objectmeta = import "../../../k8s_io/v1/objectmeta.ncl" in

{
  FunctionRevision = {
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
        name | String | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"% | default = "",
      } | doc m%"
PackagePullSecrets are named secrets in the same namespace that can be
used to fetch packages from private registries. They are also applied to
any images pulled for the package, such as a provider's controller image.
"%,
      revision | Number | doc m%"
Revision number. Indicates when the revision will be garbage collected
based on the parent's RevisionHistoryLimit.
"%,
      runtimeConfigRef | {
        apiVersion | String | doc "API version of the referent." | default = "pkg.crossplane.io/v1beta1",
        kind | String | doc "Kind of the referent." | default = "DeploymentRuntimeConfig",
        name | String | doc "Name of the RuntimeConfig.",
      } | doc m%"
RuntimeConfigRef references a RuntimeConfig resource that will be used
to configure the package runtime.
"% | default = {
        name = "default"
      },
      skipDependencyResolution | Bool | doc m%"
SkipDependencyResolution indicates to the package manager whether to skip
resolving dependencies for a package. Setting this value to true may have
unintended consequences.
Default is false.
"% | default = false,
      tlsClientSecretName | optional | String | doc m%"
TLSClientSecretName is the name of the TLS Secret that stores client
certificates of the Provider.
"%,
      tlsServerSecretName | optional | String | doc m%"
TLSServerSecretName is the name of the TLS Secret that stores server
certificates of the Provider.
"%,
    } | doc "FunctionRevisionSpec specifies configuration for a FunctionRevision.",
    status | optional | {
      appliedImageConfigRefs | optional | Array {
        name | String | doc "Name is the name of the image config.",
        reason | String | doc "Reason indicates what the image config was used for.",
      } | doc m%"
AppliedImageConfigRefs records any image configs that were applied in
reconciling this revision, and what they were used for.
"%,
      capabilities | optional | Array String | doc m%"
Capabilities of this package. Capabilities are opaque strings that
may be meaningful to package consumers.
"%,
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
      endpoint | optional | String | doc m%"
Endpoint is the gRPC endpoint where Crossplane will send
RunFunctionRequests.
"%,
      foundDependencies | optional | Number | doc "Dependency information.",
      installedDependencies | optional | Number,
      invalidDependencies | optional | Number,
      objectRefs | optional | Array {
        apiVersion | String | doc "APIVersion of the referenced object.",
        kind | String | doc "Kind of the referenced object.",
        name | String | doc "Name of the referenced object.",
        uid | optional | String | doc "UID of the referenced object.",
      } | doc "References to objects owned by PackageRevision.",
      resolvedImage | optional | String | doc m%"
ResolvedPackage is the name of the package that was installed. It may be
different from spec.image if the package path was rewritten using an
image config.
"%,
      tlsClientSecretName | optional | String | doc m%"
TLSClientSecretName is the name of the TLS Secret that stores client
certificates of the Provider.
"%,
      tlsServerSecretName | optional | String | doc m%"
TLSServerSecretName is the name of the TLS Secret that stores server
certificates of the Provider.
"%,
    } | doc "FunctionRevisionStatus represents the observed state of a FunctionRevision.",
  },
}
