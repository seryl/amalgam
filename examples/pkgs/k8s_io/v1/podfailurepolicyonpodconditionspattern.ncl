# Module: k8s.io.v1

# PodFailurePolicyOnPodConditionsPattern describes a pattern for matching an actual pod condition type.
{
  status | String | doc m%"
Specifies the required Pod condition status. To match a pod condition it is required that the specified status equals the pod condition status. Defaults to True.
"%,
  type | String | doc m%"
Specifies the required Pod condition type. To match a pod condition it is required that specified type equals the pod condition type.
"%
}