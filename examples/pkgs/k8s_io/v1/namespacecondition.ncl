# Module: k8s.io.v1

# NamespaceCondition contains details about state of namespace.
{
  lastTransitionTime
     | String
     | doc "Last time the condition transitioned from one status to another."
     | optional,
  message
     | String
     | doc "Human-readable message indicating details about last transition."
     | optional,
  reason
     | String
     | doc "Unique, one-word, CamelCase reason for the condition's last transition."
     | optional,
  status
     | String
     | doc "Status of the condition, one of True, False, Unknown.",
  type
     | String
     | doc "Type of namespace controller condition."
}