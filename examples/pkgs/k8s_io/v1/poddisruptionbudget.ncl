let objectMeta = import "./ObjectMeta.ncl" in
let podDisruptionBudgetSpec = import "./PodDisruptionBudgetSpec.ncl" in
let podDisruptionBudgetStatus = import "./PodDisruptionBudgetStatus.ncl" in

# Module: k8s.io.v1

# PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | objectMeta
     | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
     | optional,
  spec
     | podDisruptionBudgetSpec
     | doc "Specification of the desired behavior of the PodDisruptionBudget."
     | optional,
  status
     | podDisruptionBudgetStatus
     | doc "Most recently observed status of the PodDisruptionBudget."
     | optional
}