let horizontalpodautoscalercondition = import "./horizontalpodautoscalercondition.ncl" in
let metricstatus = import "./metricstatus.ncl" in

# Module: k8s.io.v2

# HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
{
  conditions | optional | Array horizontalpodautoscalercondition | doc m%"
conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
"%,
  currentMetrics | optional | Array metricstatus | doc "currentMetrics is the last read state of the metrics used by this autoscaler.",
  currentReplicas | optional | Number | doc m%"
currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
"%,
  desiredReplicas | optional | Number | doc m%"
desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
"%,
  lastScaleTime | optional | String | doc m%"
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
"%,
  observedGeneration | optional | Number | doc "observedGeneration is the most recent generation observed by this autoscaler."
}