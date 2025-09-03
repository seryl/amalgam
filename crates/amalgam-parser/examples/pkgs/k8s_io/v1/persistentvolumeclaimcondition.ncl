# Module: k8s.io.v1

# PersistentVolumeClaimCondition contains details about state of pvc
{
  lastProbeTime | optional | String | doc "lastProbeTime is the time we probed the condition.",
  lastTransitionTime | optional | String | doc m%"
lastTransitionTime is the time the condition transitioned from one status to another.
"%,
  message | optional | String | doc "message is the human-readable message indicating details about last transition.",
  reason | optional | String | doc m%"
reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "Resizing" that means the underlying persistent volume is being resized.
"%,
  status | optional | String | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=state%20of%20pvc-,conditions.status,-(string)%2C%20required
"%,
  type | optional | String | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=set%20to%20%27ResizeStarted%27.-,PersistentVolumeClaimCondition,-contains%20details%20about
"%
}
