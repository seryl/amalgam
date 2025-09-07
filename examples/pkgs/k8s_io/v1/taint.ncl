# Module: k8s.io.v1

# The node this Taint is attached to has the "effect" on any pod that does not tolerate the Taint.
{
  effect | String | doc m%"
Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
"%,
  key | String | doc "Required. The taint key to be applied to a node.",
  timeAdded | optional | String | doc m%"
TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
"%,
  value | optional | String | doc "The taint value corresponding to the taint key."
}