{
  ContainerResourceMetricSource = {
    container
       | String
       | doc "container is the name of the container in the pods of the scaling target",
    name
       | String
       | doc "name is the name of the resource in question.",
    target
       | io.k8s.api.autoscaling.v2.MetricTarget
       | doc "target specifies the target value for the given metric"
  },

  ContainerResourceMetricStatus = {
    container
       | String
       | doc "container is the name of the container in the pods of the scaling target",
    current
       | io.k8s.api.autoscaling.v2.MetricValueStatus
       | doc "current contains the current value for the given metric",
    name
       | String
       | doc "name is the name of the resource in question."
  },

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

  ExternalMetricSource = {
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector",
    target
       | io.k8s.api.autoscaling.v2.MetricTarget
       | doc "target specifies the target value for the given metric"
  },

  ExternalMetricStatus = {
    current
       | io.k8s.api.autoscaling.v2.MetricValueStatus
       | doc "current contains the current value for the given metric",
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector"
  },

  HPAScalingPolicy = {
    periodSeconds
       | Number
       | doc m%"
periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
"%,
    type
       | String
       | doc "type is used to specify the scaling policy.",
    value
       | Number
       | doc m%"
value contains the amount of change which is permitted by the policy. It must be greater than zero
"%
  },

  HPAScalingRules = {
    policies
       | Array io.k8s.api.autoscaling.v2.HPAScalingPolicy
       | doc m%"
policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
"%
       | optional,
    selectPolicy
       | String
       | doc m%"
selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
"%
       | optional,
    stabilizationWindowSeconds
       | Number
       | doc m%"
stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
"%
       | optional,
    tolerance
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc m%"
tolerance is the tolerance on the ratio between the current and desired metric value under which no updates are made to the desired number of replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not set, the default cluster-wide tolerance is applied (by default 10%).

For example, if autoscaling is configured with a memory consumption target of 100Mi, and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be triggered when the actual consumption falls below 95Mi or exceeds 101Mi.

This is an alpha field and requires enabling the HPAConfigurableTolerance feature gate.
"%
       | optional
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
       | ObjectMeta
       | doc m%"
metadata is the standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerSpec
       | doc m%"
spec is the specification for the behaviour of the autoscaler. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
"%
       | optional,
    status
       | io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerStatus
       | doc "status is the current information about the autoscaler."
       | optional
  },

  HorizontalPodAutoscalerBehavior = {
    scaleDown
       | io.k8s.api.autoscaling.v2.HPAScalingRules
       | doc m%"
scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
"%
       | optional,
    scaleUp
       | io.k8s.api.autoscaling.v2.HPAScalingRules
       | doc m%"
scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
  * increase no more than 4 pods per 60 seconds
  * double the number of pods per 60 seconds
No stabilization is used.
"%
       | optional
  },

  HorizontalPodAutoscalerCondition = {
    lastTransitionTime
       | Time
       | doc m%"
lastTransitionTime is the last time the condition transitioned from one status to another
"%
       | optional,
    message
       | String
       | doc "message is a human-readable explanation containing details about the transition"
       | optional,
    reason
       | String
       | doc "reason is the reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "status is the status of the condition (True, False, Unknown)",
    type
       | String
       | doc "type describes the current condition"
  },

  HorizontalPodAutoscalerList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.autoscaling.v2.HorizontalPodAutoscaler
       | doc "items is the list of horizontal pod autoscaler objects.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "metadata is the standard list metadata."
       | optional
  },

  HorizontalPodAutoscalerSpec = {
    behavior
       | io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerBehavior
       | doc m%"
behavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively). If not set, the default HPAScalingRules for scale up and scale down are used.
"%
       | optional,
    maxReplicas
       | Number
       | doc m%"
maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
"%,
    metrics
       | Array io.k8s.api.autoscaling.v2.MetricSpec
       | doc m%"
metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond. If not set, the default metric will be set to 80% average CPU utilization.
"%
       | optional,
    minReplicas
       | Number
       | doc m%"
minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
"%
       | optional,
    scaleTargetRef
       | io.k8s.api.autoscaling.v2.CrossVersionObjectReference
       | doc m%"
scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
"%
  },

  HorizontalPodAutoscalerStatus = {
    conditions
       | Array io.k8s.api.autoscaling.v2.HorizontalPodAutoscalerCondition
       | doc m%"
conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
"%
       | optional,
    currentMetrics
       | Array io.k8s.api.autoscaling.v2.MetricStatus
       | doc "currentMetrics is the last read state of the metrics used by this autoscaler."
       | optional,
    currentReplicas
       | Number
       | doc m%"
currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
"%
       | optional,
    desiredReplicas
       | Number
       | doc m%"
desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
"%,
    lastScaleTime
       | Time
       | doc m%"
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
"%
       | optional,
    observedGeneration
       | Number
       | doc "observedGeneration is the most recent generation observed by this autoscaler."
       | optional
  },

  MetricIdentifier = {
    name
       | String
       | doc "name is the name of the given metric",
    selector
       | LabelSelector
       | doc m%"
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
"%
       | optional
  },

  MetricSpec = {
    containerResource
       | io.k8s.api.autoscaling.v2.ContainerResourceMetricSource
       | doc m%"
containerResource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%
       | optional,
    external
       | io.k8s.api.autoscaling.v2.ExternalMetricSource
       | doc m%"
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
"%
       | optional,
    object
       | io.k8s.api.autoscaling.v2.ObjectMetricSource
       | doc m%"
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
"%
       | optional,
    pods
       | io.k8s.api.autoscaling.v2.PodsMetricSource
       | doc m%"
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
"%
       | optional,
    resource
       | io.k8s.api.autoscaling.v2.ResourceMetricSource
       | doc m%"
resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%
       | optional,
    type
       | String
       | doc m%"
type is the type of metric source.  It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object.
"%
  },

  MetricStatus = {
    containerResource
       | io.k8s.api.autoscaling.v2.ContainerResourceMetricStatus
       | doc m%"
container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%
       | optional,
    external
       | io.k8s.api.autoscaling.v2.ExternalMetricStatus
       | doc m%"
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
"%
       | optional,
    object
       | io.k8s.api.autoscaling.v2.ObjectMetricStatus
       | doc m%"
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
"%
       | optional,
    pods
       | io.k8s.api.autoscaling.v2.PodsMetricStatus
       | doc m%"
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
"%
       | optional,
    resource
       | io.k8s.api.autoscaling.v2.ResourceMetricStatus
       | doc m%"
resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%
       | optional,
    type
       | String
       | doc m%"
type is the type of metric source.  It will be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
"%
  },

  MetricTarget = {
    averageUtilization
       | Number
       | doc m%"
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
"%
       | optional,
    averageValue
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc m%"
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
"%
       | optional,
    type
       | String
       | doc "type represents whether the metric type is Utilization, Value, or AverageValue",
    value
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc "value is the target value of the metric (as a quantity)."
       | optional
  },

  MetricValueStatus = {
    averageUtilization
       | Number
       | doc m%"
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
"%
       | optional,
    averageValue
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc m%"
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
"%
       | optional,
    value
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc "value is the current value of the metric (as a quantity)."
       | optional
  },

  ObjectMetricSource = {
    describedObject
       | io.k8s.api.autoscaling.v2.CrossVersionObjectReference
       | doc m%"
describedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector",
    target
       | io.k8s.api.autoscaling.v2.MetricTarget
       | doc "target specifies the target value for the given metric"
  },

  ObjectMetricStatus = {
    current
       | io.k8s.api.autoscaling.v2.MetricValueStatus
       | doc "current contains the current value for the given metric",
    describedObject
       | io.k8s.api.autoscaling.v2.CrossVersionObjectReference
       | doc m%"
DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector"
  },

  PodsMetricSource = {
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector",
    target
       | io.k8s.api.autoscaling.v2.MetricTarget
       | doc "target specifies the target value for the given metric"
  },

  PodsMetricStatus = {
    current
       | io.k8s.api.autoscaling.v2.MetricValueStatus
       | doc "current contains the current value for the given metric",
    metric
       | io.k8s.api.autoscaling.v2.MetricIdentifier
       | doc "metric identifies the target metric by name and selector"
  },

  ResourceMetricSource = {
    name
       | String
       | doc "name is the name of the resource in question.",
    target
       | io.k8s.api.autoscaling.v2.MetricTarget
       | doc "target specifies the target value for the given metric"
  },

  ResourceMetricStatus = {
    current
       | io.k8s.api.autoscaling.v2.MetricValueStatus
       | doc "current contains the current value for the given metric",
    name
       | String
       | doc "name is the name of the resource in question."
  }
}