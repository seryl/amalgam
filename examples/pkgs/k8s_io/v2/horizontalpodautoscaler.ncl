let objectMeta = import "../v1/ObjectMeta.ncl" in
let horizontalPodAutoscalerSpec = import "./HorizontalPodAutoscalerSpec.ncl" in
let horizontalPodAutoscalerStatus = import "./HorizontalPodAutoscalerStatus.ncl" in

# Module: k8s.io.v2

# HorizontalPodAutoscaler is the configuration for a horizontal pod autoscaler, which automatically manages the replica count of any resource implementing the scale subresource based on the metrics specified.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectMeta | doc m%"
metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | horizontalPodAutoscalerSpec | doc m%"
spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
"%,
  status | optional | horizontalPodAutoscalerStatus | doc "status is the current information about the autoscaler."
}