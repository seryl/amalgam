# Module: k8s.io.v1

# NamespaceCondition contains details about state of namespace.
{
  lastTransitionTime | optional | String | doc "Last time the condition transitioned from one status to another.",
  message | optional | String | doc "Human-readable message indicating details about last transition.",
  reason | optional | String | doc "Unique, one-word, CamelCase reason for the condition's last transition.",
  status | optional | String | doc "Status of the condition, one of True, False, Unknown.",
  type | optional | String | doc "Type of namespace controller condition."
}
