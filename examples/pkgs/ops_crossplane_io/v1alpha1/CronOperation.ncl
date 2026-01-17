let metav1 = import "../../k8s_io/apimachinery.pkg.apis/meta/v1/mod.ncl" in

# Module: ops.crossplane.io.v1alpha1

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
    concurrencyPolicy
      | String
      | std.contract.from_predicate (fun value => std.array.elem value ["Allow", "Forbid", "Replace"])
      | doc m%"
ConcurrencyPolicy specifies how to treat concurrent executions of an
operation.
"%
      = "Allow",
    failedHistoryLimit
      | Number
      | doc "FailedHistoryLimit is the number of failed Operations to retain."
      = 1,
    operationTemplate
      | {
      metadata
        | metav1.ObjectMeta
        | doc "Standard object metadata."
        | optional,
      spec
        | {
        mode
          | String
          | std.contract.from_predicate (fun value => std.array.elem value ["Pipeline"])
          | doc m%"
Mode controls what type or "mode" of operation will be used.
"Pipeline" indicates that an Operation specifies a pipeline of
functions, each of which is responsible for implementing its logic.
"%
          = "Pipeline",
        pipeline
          | Array {
          credentials
            | Array {
            name
              | String
              | doc "Name of this set of credentials.",
            secretRef
              | {
              name
                | String
                | doc "Name of the secret.",
              namespace
                | String
                | doc "Namespace of the secret."
            }
              | doc m%"
A SecretRef is a reference to a secret containing credentials that should
be supplied to the function.
"%
              | optional,
            source
              | String
              | std.contract.from_predicate (fun value => std.array.elem value ["None", "Secret"])
              | doc "Source of the function credentials."
          }
            | doc "Credentials are optional credentials that the operation function needs."
            | optional,
          functionRef
            | {
            name
              | String
              | doc "Name of the referenced function."
          }
            | doc m%"
FunctionRef is a reference to the function this step should
execute.
"%,
          input
            | { .. }
            | doc m%"
Input is an optional, arbitrary Kubernetes resource (i.e. a resource
with an apiVersion and kind) that will be passed to the unction as
the 'input' of its RunFunctionRequest.
"%
            | optional,
          requirements
            | {
            requiredResources
              | Array {
              apiVersion
                | String
                | doc "APIVersion of resources to select.",
              kind
                | String
                | doc "Kind of resources to select.",
              matchLabels
                | { _ : String }
                | doc m%"
MatchLabels matches resources by label selector. Only one of Name or
MatchLabels may be specified.
"%
                | optional,
              name
                | String
                | doc m%"
Name matches a single resource by name. Only one of Name or
MatchLabels may be specified.
"%
                | optional,
              namespace
                | String
                | doc "Namespace to search for resources. Optional for cluster-scoped resources."
                | optional,
              requirementName
                | String
                | doc m%"
RequirementName uniquely identifies this group of resources.
This name will be used as the key in RunFunctionRequest.required_resources.
"%
            }
              | doc m%"
RequiredResources that will be fetched before this pipeline step
is called for the first time.
"%
              | optional
          }
            | doc m%"
Requirements are resource requirements that will be satisfied before
this pipeline step is called for the first time. This allows
pre-populating required resources without requiring a function to
request them first.
"%
            | optional,
          step
            | String
            | doc "Step name. Must be unique within its Pipeline."
        }
          | std.contract.from_predicate (fun value => std.array.length value >= 1 && std.array.length value <= 99)
          | doc m%"
Pipeline is a list of operation function steps that will be used when
this operation runs.
"%,
        retryLimit
          | Number
          | doc m%"
RetryLimit configures how many times the operation may fail. When the
failure limit is exceeded, the operation will not be retried.
"%
          | optional
      }
        | doc "Spec is the specification of the Operation to be created."
    }
      | doc "OperationTemplate is the template for the Operation to be created.",
    schedule
      | String
      | doc "Schedule is the cron schedule for the operation.",
    startingDeadlineSeconds
      | Number
      | doc m%"
StartingDeadlineSeconds is the deadline in seconds for starting the
operation if it misses its scheduled time for any reason.
"%
      | optional,
    successfulHistoryLimit
      | Number
      | doc "SuccessfulHistoryLimit is the number of successful Operations to retain."
      = 3
  }
    | doc "CronOperationSpec specifies the desired state of a CronOperation."
    | optional,
  status
    | {
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
    lastScheduleTime
      | String
      | std.contract.from_predicate (fun value => std.string.is_match "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}" value)
      | doc "LastScheduleTime is the last time the CronOperation was scheduled."
      | optional,
    lastSuccessfulTime
      | String
      | std.contract.from_predicate (fun value => std.string.is_match "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}" value)
      | doc m%"
LastSuccessfulTime is the last time the CronOperation was successfully
completed.
"%
      | optional,
    runningOperationRefs
      | Array {
      name
        | String
        | doc "Name of the active operation."
    }
      | doc "RunningOperationRefs is a list of currently running Operations."
      | optional
  }
    | doc "CronOperationStatus represents the observed state of a CronOperation."
    | optional
}