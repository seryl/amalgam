# Module: k8s.io.v1.replicationcontrollercondition

{
  # ReplicationControllerCondition describes the state of a replication controller at a certain point.
  ReplicationControllerCondition = {
    lastTransitionTime | optional | String | doc "The last time the condition transitioned from one status to another.",
    message | optional | String | doc "A human readable message indicating details about the transition.",
    reason | optional | String | doc "The reason for the condition's last transition.",
    status | String | doc "Status of the condition, one of True, False, Unknown.",
    type | String | doc "Type of replication controller condition.",
  },
}
