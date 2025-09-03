let objectmeta = import "../../../k8s_io/v1/objectmeta.ncl" in


{
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
  metadata | optional | objectmeta.ObjectMeta,
  spec | optional | {
    compositeTypeRef | optional | {
      apiVersion | optional | String | doc "APIVersion of the type.",
      kind | optional | String | doc "Kind of the type."
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
        name | optional | String | doc "Name of this set of credentials.",
        secretRef | optional | {
          name | optional | String | doc "Name of the secret.",
          namespace | optional | String | doc "Namespace of the secret."
        } | doc m%"
A SecretRef is a reference to a secret containing credentials that should
be supplied to the function.
"%,
        source | optional | String | doc "Source of the function credentials."
      } | doc "Credentials are optional credentials that the function needs.",
      functionRef | optional | {
        name | optional | String | doc "Name of the referenced Function."
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
          apiVersion | optional | String | doc "APIVersion of the required resource.",
          kind | optional | String | doc "Kind of the required resource.",
          matchLabels | optional | {
          } | doc m%"
MatchLabels specifies the set of labels to match for finding the
required resource. When specified, Name is ignored.
"%,
          name | optional | String | doc "Name of the required resource.",
          namespace | optional | String | doc "Namespace of the required resource if it is namespaced.",
          requirementName | optional | String | doc m%"
RequirementName is the unique name to identify this required resource
in the Required Resources map in the function request.
"%
        } | doc m%"
RequiredResources is a list of resources that must be fetched before
this function is called.
"%
      } | doc m%"
Requirements are resource requirements that will be satisfied before
this pipeline step is called for the first time. This allows
pre-populating required resources without requiring a function to
request them first.
"%,
      step | optional | String | doc "Step name. Must be unique within its Pipeline."
    } | doc m%"
Pipeline is a list of composition function steps that will be used when a
composite resource referring to this composition is created. One of
resources and pipeline must be specified - you cannot specify both.
The Pipeline is only used by the "Pipeline" mode of Composition. It is
ignored by other modes.
"%,
    writeConnectionSecretsToNamespace | optional | String | doc m%"
WriteConnectionSecretsToNamespace specifies the namespace in which the
connection secrets of composite resource dynamically provisioned using
this composition will be created.
"%
  } | doc "CompositionSpec specifies desired state of a composition."
}