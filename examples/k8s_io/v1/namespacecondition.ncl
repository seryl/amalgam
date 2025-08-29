# Module: k8s.io.v1.namespacecondition

{
  # NamespaceCondition contains details about state of namespace.
  NamespaceCondition = {
    lastTransitionTime | optional | String,
    message | optional | String,
    reason | optional | String,
    status | String | doc "Status of the condition, one of True, False, Unknown.",
    type | String | doc "Type of namespace controller condition.",
  },
}
