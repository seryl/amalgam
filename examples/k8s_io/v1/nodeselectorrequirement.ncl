# Module: k8s.io.v1.nodeselectorrequirement

{
  # A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
  NodeSelectorRequirement = {
    key | String | doc "The label key that the selector applies to.",
    operator | String | doc m%"
Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
    values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
"%,
  },
}
