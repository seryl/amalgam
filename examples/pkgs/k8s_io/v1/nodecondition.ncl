# Module: k8s.io.v1.nodecondition

{
  # NodeCondition contains condition information for a node.
  NodeCondition = {
    lastHeartbeatTime | optional | String | doc "Last time we got an update on a given condition.",
    lastTransitionTime | optional | String | doc "Last time the condition transit from one status to another.",
    message | optional | String | doc "Human readable message indicating details about last transition.",
    reason | optional | String | doc "(brief) reason for the condition's last transition.",
    status | String | doc "Status of the condition, one of True, False, Unknown.",
    type | String | doc "Type of node condition.",
  },
}
