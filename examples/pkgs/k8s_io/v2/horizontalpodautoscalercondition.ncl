# Module: k8s.io.v2

# HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
{
  lastTransitionTime | optional | String | doc m%"
lastTransitionTime is the last time the condition transitioned from one status to another
"%,
  message | optional | String | doc "message is a human-readable explanation containing details about the transition",
  reason | optional | String | doc "reason is the reason for the condition's last transition.",
  status | String | doc "status is the status of the condition (True, False, Unknown)",
  type | String | doc "type describes the current condition"
}