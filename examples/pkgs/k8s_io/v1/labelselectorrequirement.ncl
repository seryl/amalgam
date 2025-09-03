# Module: k8s.io.v1

# A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
{
  key | optional | String | doc "key is the label key that the selector applies to.",
  operator | optional | String | doc m%"
operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
"%
}