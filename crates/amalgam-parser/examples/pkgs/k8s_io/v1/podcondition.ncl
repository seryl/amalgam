# Module: k8s.io.v1

# PodCondition contains details for the current condition of this pod.
{
  lastProbeTime | optional | String | doc "Last time we probed the condition.",
  lastTransitionTime | optional | String | doc "Last time the condition transitioned from one status to another.",
  message | optional | String | doc "Human-readable message indicating details about last transition.",
  observedGeneration | optional | Number | doc m%"
If set, this represents the .metadata.generation that the pod condition was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
"%,
  reason | optional | String | doc "Unique, one-word, CamelCase reason for the condition's last transition.",
  status | optional | String | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%,
  type | optional | String | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%
}
