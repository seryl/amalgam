# ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
{
  current | optional | MetricValueStatus | doc "current contains the current value for the given metric",
  metric | optional | MetricIdentifier | doc "metric identifies the target metric by name and selector",
}
