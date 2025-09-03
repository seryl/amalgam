# Module: k8s.io.v1

# A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.
{
  key | optional | String | doc "The label key that the selector applies to.",
  values | optional | Array String | doc m%"
An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
"%
}