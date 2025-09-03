# Describes the state of a migration at a certain point.
{
  lastUpdateTime | optional | String | doc "The last time this condition was updated.",
  message | optional | String | doc "A human readable message indicating details about the transition.",
  reason | optional | String | doc "The reason for the condition's last transition.",
  status | optional | String | doc "Status of the condition, one of True, False, Unknown.",
  type | optional | String | doc "Type of the condition.",
}
