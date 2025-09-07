let metricValueStatus = import "./MetricValueStatus.ncl" in
let metricIdentifier = import "./MetricIdentifier.ncl" in

# Module: k8s.io.v2

# ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
{
  current | metricValueStatus | doc "current contains the current value for the given metric",
  metric | metricIdentifier | doc "metric identifies the target metric by name and selector"
}