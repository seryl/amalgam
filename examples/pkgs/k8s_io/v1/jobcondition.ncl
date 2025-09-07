# Module: k8s.io.v1

# JobCondition describes current state of a job.
{
  lastProbeTime
     | String
     | doc "Last time the condition was checked."
     | optional,
  lastTransitionTime
     | String
     | doc "Last time the condition transit from one status to another."
     | optional,
  message
     | String
     | doc "Human readable message indicating details about last transition."
     | optional,
  reason
     | String
     | doc "(brief) reason for the condition's last transition."
     | optional,
  status
     | String
     | doc "Status of the condition, one of True, False, Unknown.",
  type
     | String
     | doc "Type of job condition, Complete or Failed."
}