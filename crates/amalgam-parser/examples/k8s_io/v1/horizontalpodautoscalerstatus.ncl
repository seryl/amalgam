# current status of a horizontal pod autoscaler
{
  currentCPUUtilizationPercentage | optional | Number | doc m%"
currentCPUUtilizationPercentage is the current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.
"%,
  currentReplicas | optional | Number | doc m%"
currentReplicas is the current number of replicas of pods managed by this autoscaler.
"%,
  desiredReplicas | optional | Number | doc m%"
desiredReplicas is the  desired number of replicas of pods managed by this autoscaler.
"%,
  lastScaleTime | optional | String | doc m%"
lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed.
"%,
  observedGeneration | optional | Number | doc "observedGeneration is the most recent generation observed by this autoscaler.",
}
