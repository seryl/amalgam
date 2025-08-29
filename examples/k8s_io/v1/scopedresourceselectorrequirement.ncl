# Module: k8s.io.v1.scopedresourceselectorrequirement

{
  # A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.
  ScopedResourceSelectorRequirement = {
    operator | String | doc m%"
Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
"%,
    scopeName | String | doc "The name of the scope that the selector applies to.",
    values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
"%,
  },
}
