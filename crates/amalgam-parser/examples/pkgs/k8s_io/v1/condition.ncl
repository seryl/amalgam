# Module: k8s.io.v1

# Condition contains details for one aspect of the current state of this API Resource.
{
  lastTransitionTime | optional | String | doc m%"
lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
"%,
  message | optional | String | doc m%"
message is a human readable message indicating details about the transition. This may be an empty string.
"%,
  observedGeneration | optional | Number | doc m%"
observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
"%,
  reason | optional | String | doc m%"
reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
"%,
  status | optional | String | doc "status of the condition, one of True, False, Unknown.",
  type | optional | String | doc "type of condition in CamelCase or in foo.example.com/CamelCase."
}
