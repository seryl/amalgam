# Module: k8s.io.v1

# StatefulSetCondition describes the state of a statefulset at a certain point.
{
  lastTransitionTime | optional | String | doc "Last time the condition transitioned from one status to another.",
  message | optional | String | doc "A human readable message indicating details about the transition.",
  reason | optional | String | doc "The reason for the condition's last transition.",
  status | String | doc "Status of the condition, one of True, False, Unknown.",
  type | String | doc "Type of statefulset condition."
}