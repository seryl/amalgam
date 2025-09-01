# Module: k8s.io.v1.componentcondition

{
  # Information about the condition of a component.
  ComponentCondition = {
    error | optional | String | doc "Condition error code for a component. For example, a health check error code.",
    message | optional | String | doc m%"
Message about the condition for a component. For example, information about a health check.
"%,
    status | String | doc m%"
Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
"%,
    type | String | doc "Type of condition for a component. Valid value: \"Healthy\"",
  },
}
