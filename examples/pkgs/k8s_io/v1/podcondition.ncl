# Module: k8s.io.v1

# PodCondition contains details for the current condition of this pod.
{
  lastProbeTime
     | String
     | doc "Last time we probed the condition."
     | optional,
  lastTransitionTime
     | String
     | doc "Last time the condition transitioned from one status to another."
     | optional,
  message
     | String
     | doc "Human-readable message indicating details about last transition."
     | optional,
  observedGeneration
     | Number
     | doc m%"
If set, this represents the .metadata.generation that the pod condition was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
"%
     | optional,
  reason
     | String
     | doc "Unique, one-word, CamelCase reason for the condition's last transition."
     | optional,
  status
     | String
     | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%,
  type
     | String
     | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%
}