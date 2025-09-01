# Module: k8s.io.v1.podreadinessgate

{
  # PodReadinessGate contains the reference to a pod condition
  PodReadinessGate = {
    conditionType | optional | String | doc m%"
ConditionType refers to a condition in the pod's condition list with matching type.
"%,
  },
}
