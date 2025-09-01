# Module: k8s.io.v1.podcondition

{
  # PodCondition contains details for the current condition of this pod.
  PodCondition = {
    lastProbeTime | optional | String | doc "Last time we probed the condition.",
    lastTransitionTime | optional | String | doc "Last time the condition transitioned from one status to another.",
    message | optional | String | doc "Human-readable message indicating details about last transition.",
    reason | optional | String | doc "Unique, one-word, CamelCase reason for the condition's last transition.",
    status | String | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%,
    type | String | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%,
  },
}
