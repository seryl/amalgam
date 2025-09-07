# Module: k8s.io.v1alpha1

# Describes the state of a migration at a certain point.
{
  lastUpdateTime
     | String
     | doc "The last time this condition was updated."
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
     | doc "Type of the condition."
}