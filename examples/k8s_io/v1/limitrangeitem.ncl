# Module: k8s.io.v1.limitrangeitem

{
  # LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
  LimitRangeItem = {
    default | optional | { .. } | doc m%"
Default resource requirement limit value by resource name if resource limit is omitted.
"%,
    defaultRequest | optional | { .. } | doc m%"
DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
"%,
    max | optional | { .. } | doc "Max usage constraints on this kind by resource name.",
    maxLimitRequestRatio | optional | { .. } | doc m%"
MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
"%,
    min | optional | { .. } | doc "Min usage constraints on this kind by resource name.",
    type | String | doc "Type of resource that this limit applies to.",
  },
}
