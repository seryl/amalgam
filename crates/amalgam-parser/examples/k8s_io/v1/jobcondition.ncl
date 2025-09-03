# JobCondition describes current state of a job.
{
  lastProbeTime | optional | String | doc "Last time the condition was checked.",
  lastTransitionTime | optional | String | doc "Last time the condition transit from one status to another.",
  message | optional | String | doc "Human readable message indicating details about last transition.",
  reason | optional | String | doc "(brief) reason for the condition's last transition.",
  status | optional | String | doc "Status of the condition, one of True, False, Unknown.",
  type | optional | String | doc "Type of job condition, Complete or Failed.",
}
