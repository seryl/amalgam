let metricValueStatus = import "./MetricValueStatus.ncl" in
let metricIdentifier = import "./MetricIdentifier.ncl" in

# Module: k8s.io.v2

# PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
{
  current | metricValueStatus | doc "current contains the current value for the given metric",
  metric | metricIdentifier | doc "metric identifies the target metric by name and selector"
}