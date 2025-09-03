let v1Module = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in

# Module: apiextensions.crossplane.io.v1beta1

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
     | v1Module.ObjectMeta
     | optional,
  spec
     | {
    by
       | {
      apiVersion
         | String
         | doc "API version of the referent."
         | optional,
      kind
         | String
         | doc m%"
Kind of the referent.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
         | optional,
      resourceRef
         | {
        name
           | String
           | doc "Name of the referent."
      }
         | doc "Reference to the resource."
         | optional,
      resourceSelector
         | {
        matchControllerRef
           | Bool
           | doc m%"
MatchControllerRef ensures an object with the same controller reference
as the selecting object is selected.
"%
           | optional,
        matchLabels
           | {
        }
           | doc "MatchLabels ensures an object with matching labels is selected."
           | optional
      }
         | doc m%"
Selector to the resource.
This field will be ignored if ResourceRef is set.
"%
         | optional
    }
       | doc "By is the resource that is \"using the other resource\"."
       | optional,
    of
       | {
      apiVersion
         | String
         | doc "API version of the referent."
         | optional,
      kind
         | String
         | doc m%"
Kind of the referent.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
         | optional,
      resourceRef
         | {
        name
           | String
           | doc "Name of the referent."
      }
         | doc "Reference to the resource."
         | optional,
      resourceSelector
         | {
        matchControllerRef
           | Bool
           | doc m%"
MatchControllerRef ensures an object with the same controller reference
as the selecting object is selected.
"%
           | optional,
        matchLabels
           | {
        }
           | doc "MatchLabels ensures an object with matching labels is selected."
           | optional
      }
         | doc m%"
Selector to the resource.
This field will be ignored if ResourceRef is set.
"%
         | optional
    }
       | doc "Of is the resource that is \"being used\".",
    reason
       | String
       | doc "Reason is the reason for blocking deletion of the resource."
       | optional,
    replayDeletion
       | Bool
       | doc m%"
ReplayDeletion will trigger a deletion on the used resource during the deletion of the usage itself, if it was attempted to be deleted at least once.
"%
       | optional
  }
     | doc "UsageSpec defines the desired state of Usage.",
  status
     | {
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
     | doc "UsageStatus defines the observed state of Usage."
     | optional
}