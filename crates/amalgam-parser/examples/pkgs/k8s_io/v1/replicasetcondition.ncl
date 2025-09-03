# Module: k8s.io.v1

# ReplicaSetCondition describes the state of a replica set at a certain point.
{
  lastTransitionTime | optional | String | doc "The last time the condition transitioned from one status to another.",
  message | optional | String | doc "A human readable message indicating details about the transition.",
  reason | optional | String | doc "The reason for the condition's last transition.",
  status | optional | String | doc "Status of the condition, one of True, False, Unknown.",
  type | optional | String | doc "Type of replica set condition."
}
