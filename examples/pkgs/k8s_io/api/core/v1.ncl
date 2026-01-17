# Module: k8s.io.v1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let listMeta = metav1.ListMeta in
let objectMeta = metav1.ObjectMeta in

{
  CrossVersionObjectReference = {
    apiVersion
      | String
      | doc "apiVersion is the API version of the referent"
      | optional,
    kind
      | String
      | doc m%"
kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    name
      | String
      | doc m%"
name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
  },

  HorizontalPodAutoscaler = {
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
      | HorizontalPodAutoscalerSpec
      | doc m%"
spec defines the behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
"%
      | optional,
    status
      | HorizontalPodAutoscalerStatus
      | doc "status is the current information about the autoscaler."
      | optional
  },

  HorizontalPodAutoscalerList = {
    apiVersion
      | String
      | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
      | optional,
    items
      | Array HorizontalPodAutoscaler
      | doc "items is the list of horizontal pod autoscaler objects.",
    kind
      | String
      | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
      | optional,
    metadata
      | listMeta
      | doc "Standard list metadata."
      | optional
  },

  HorizontalPodAutoscalerSpec = {
    maxReplicas
      | Number
      | doc m%"
maxReplicas is the upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
"%,
    minReplicas
      | Number
      | doc m%"
minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
"%
      | optional,
    scaleTargetRef
      | CrossVersionObjectReference
      | doc m%"
reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
"%,
    targetCPUUtilizationPercentage
      | Number
      | doc m%"
targetCPUUtilizationPercentage is the target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
"%
      | optional
  },

  HorizontalPodAutoscalerStatus = {
    currentCPUUtilizationPercentage
      | Number
      | doc m%"
currentCPUUtilizationPercentage is the current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.
"%
      | optional,
    currentReplicas
      | Number
      | doc m%"
currentReplicas is the current number of replicas of pods managed by this autoscaler.
"%,
    desiredReplicas
      | Number
      | doc m%"
desiredReplicas is the  desired number of replicas of pods managed by this autoscaler.
"%,
    lastScaleTime
      | String
      | doc m%"
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed.
"%
      | optional,
    observedGeneration
      | Number
      | doc "observedGeneration is the most recent generation observed by this autoscaler."
      | optional
  },

  Scale = {
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
Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
"%
      | optional,
    spec
      | ScaleSpec
      | doc m%"
spec defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
"%
      | optional,
    status
      | ScaleStatus
      | doc m%"
status is the current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
"%
      | optional
  },

  ScaleSpec = {
    replicas
      | Number
      | doc "replicas is the desired number of instances for the scaled object."
      | optional
  },

  ScaleStatus = {
    replicas
      | Number
      | doc "replicas is the actual number of observed instances of the scaled object.",
    selector
      | String
      | doc m%"
selector is the label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%
      | optional
  }
}
