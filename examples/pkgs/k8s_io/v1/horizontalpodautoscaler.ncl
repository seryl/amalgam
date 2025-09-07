let objectMeta = import "./ObjectMeta.ncl" in
let horizontalPodAutoscalerSpec = import "./HorizontalPodAutoscalerSpec.ncl" in
let horizontalPodAutoscalerStatus = import "./HorizontalPodAutoscalerStatus.ncl" in

# Module: k8s.io.v1

# configuration of a horizontal pod autoscaler.
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
Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
     | optional,
  spec
     | horizontalPodAutoscalerSpec
     | doc m%"
spec defines the behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
"%
     | optional,
  status
     | horizontalPodAutoscalerStatus
     | doc "status is the current information about the autoscaler."
     | optional
}