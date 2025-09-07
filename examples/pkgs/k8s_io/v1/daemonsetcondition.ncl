# Module: k8s.io.v1

# DaemonSetCondition describes the state of a DaemonSet at a certain point.
{
  lastTransitionTime
     | String
     | doc "Last time the condition transitioned from one status to another."
     | optional,
  message
     | String
     | doc "A human readable message indicating details about the transition."
     | optional,
  reason
     | String
     | doc "The reason for the condition's last transition."
     | optional,
  status
     | String
     | doc "Status of the condition, one of True, False, Unknown.",
  type
     | String
     | doc "Type of DaemonSet condition."
}