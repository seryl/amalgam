# Module: k8s.io.v1.persistentvolumeclaimcondition

{
  # PersistentVolumeClaimCondition contains details about state of pvc
  PersistentVolumeClaimCondition = {
    lastProbeTime | optional | String | doc "lastProbeTime is the time we probed the condition.",
    lastTransitionTime | optional | String | doc m%"
lastTransitionTime is the time the condition transitioned from one status to another.
"%,
    message | optional | String | doc "message is the human-readable message indicating details about last transition.",
    reason | optional | String | doc m%"
reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "Resizing" that means the underlying persistent volume is being resized.
"%,
    status | String,
    type | String,
  },
}
