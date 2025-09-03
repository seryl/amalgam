let metricvaluestatus = import "./metricvaluestatus.ncl" in
let metricidentifier = import "./metricidentifier.ncl" in

# Module: k8s.io.v2

# ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
{
  current | optional | metricvaluestatus | doc "current contains the current value for the given metric",
  metric | optional | metricidentifier | doc "metric identifies the target metric by name and selector"
}