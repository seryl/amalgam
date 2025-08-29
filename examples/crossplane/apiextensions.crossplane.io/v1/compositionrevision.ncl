# Module: compositionrevision.apiextensions.crossplane.io

let k8s_io_objectmeta = import "../../../k8s_io/v1/objectmeta.ncl" in

{
  CompositionRevision = {
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
      compositeTypeRef | {
        apiVersion | String | doc "APIVersion of the type.",
        kind | String | doc "Kind of the type.",
      } | doc m%"
CompositeTypeRef specifies the type of composite resource that this
composition is compatible with.
"%,
      mode | String | doc m%"
Mode controls what type or "mode" of Composition will be used.

"Pipeline" indicates that a Composition specifies a pipeline of
functions, each of which is responsible for producing composed
resources that Crossplane should create or update.
"% | default = "Pipeline",
      pipeline | optional | Array {
        credentials | optional | Array {
          name | String | doc "Name of this set of credentials.",
          secretRef | optional | {
            name | String | doc "Name of the secret.",
            namespace | String | doc "Namespace of the secret.",
          } | doc m%"
A SecretRef is a reference to a secret containing credentials that should
be supplied to the function.
"%,
          source | String | doc "Source of the function credentials.",
        } | doc "Credentials are optional credentials that the function needs.",
        functionRef | {
          name | String | doc "Name of the referenced Function.",
        } | doc m%"
FunctionRef is a reference to the function this step should
execute.
"%,
        input | optional | {
        } | doc m%"
Input is an optional, arbitrary Kubernetes resource (i.e. a resource
with an apiVersion and kind) that will be passed to the function as
the 'input' of its RunFunctionRequest.
"%,
        requirements | optional | {
          requiredResources | optional | Array {
            apiVersion | String | doc "APIVersion of the required resource.",
            kind | String | doc "Kind of the required resource.",
            matchLabels | optional | {
            } | doc m%"
MatchLabels specifies the set of labels to match for finding the
required resource. When specified, Name is ignored.
"%,
            name | optional | String | doc "Name of the required resource.",
            namespace | optional | String | doc "Namespace of the required resource if it is namespaced.",
            requirementName | String | doc m%"
RequirementName is the unique name to identify this required resource
in the Required Resources map in the function request.
"%,
          } | doc m%"
RequiredResources is a list of resources that must be fetched before
this function is called.
"%,
        } | doc m%"
Requirements are resource requirements that will be satisfied before
this pipeline step is called for the first time. This allows
pre-populating required resources without requiring a function to
request them first.
"%,
        step | String | doc "Step name. Must be unique within its Pipeline.",
      } | doc m%"
Pipeline is a list of function steps that will be used when a
composite resource referring to this composition is created.

The Pipeline is only used by the "Pipeline" mode of Composition. It is
ignored by other modes.
"%,
      revision | Number | doc m%"
Revision number. Newer revisions have larger numbers.

This number can change. When a Composition transitions from state A
-> B -> A there will be only two CompositionRevisions. Crossplane will
edit the original CompositionRevision to change its revision number from
0 to 2.
"%,
      writeConnectionSecretsToNamespace | optional | String | doc m%"
WriteConnectionSecretsToNamespace specifies the namespace in which the
connection secrets of composite resource dynamically provisioned using
this composition will be created.
"%,
    } | doc m%"
CompositionRevisionSpec specifies the desired state of the composition
revision.
"%,
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
    } | doc m%"
CompositionRevisionStatus shows the observed state of the composition
revision.
"%,
  },
}
