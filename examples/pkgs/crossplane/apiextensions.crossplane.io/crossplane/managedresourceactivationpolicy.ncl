let objectMeta = import "../../../../k8s_io/v1/ObjectMeta.ncl" in


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
     | objectMeta.ObjectMeta
     | optional,
  spec
     | {
    activate
       | Array String
       | doc m%"
Activations is an array of MRD names to activate. Supports wildcard
prefixes (like `*.aws.crossplane.io`) but not full regular expressions.
"%
  }
     | doc m%"
ManagedResourceActivationPolicySpec specifies the desired activation state of ManagedResourceDefinitions.
"%
     | optional,
  status
     | {
    activated
       | Array String
       | doc "Activated names the ManagedResourceDefinitions this policy has activated."
       | optional,
    conditions
       | Array {
      lastTransitionTime
         | String
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
      type
         | String
         | doc m%"
Type of this condition. At most one of each condition type may apply to
a resource at any point in time.
"%
    }
       | doc "Conditions of the resource."
       | optional
  }
     | doc "ManagedResourceActivationPolicyStatus shows the observed state of the policy."
     | optional
}