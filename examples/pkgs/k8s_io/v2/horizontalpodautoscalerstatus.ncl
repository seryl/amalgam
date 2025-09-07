let horizontalPodAutoscalerCondition = import "./HorizontalPodAutoscalerCondition.ncl" in
let metricStatus = import "./MetricStatus.ncl" in

# Module: k8s.io.v2

# HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
{
  conditions
     | Array horizontalPodAutoscalerCondition
     | doc m%"
conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
"%
     | optional,
  currentMetrics
     | Array metricStatus
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
     | String
     | doc m%"
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
"%
     | optional,
  observedGeneration
     | Number
     | doc "observedGeneration is the most recent generation observed by this autoscaler."
     | optional
}