# Module: k8s.io.v1

# PersistentVolumeStatus is the current status of a persistent volume.
{
  lastPhaseTransitionTime | optional | String | doc m%"
lastPhaseTransitionTime is the time the phase transitioned from one to another and automatically resets to current time everytime a volume phase transitions.
"%,
  message | optional | String | doc m%"
message is a human-readable message indicating details about why the volume is in this state.
"%,
  phase | optional | String | doc m%"
phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
"%,
  reason | optional | String | doc m%"
reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
"%
}
