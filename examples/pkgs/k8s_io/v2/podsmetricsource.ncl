let metricIdentifier = import "./MetricIdentifier.ncl" in
let metricTarget = import "./MetricTarget.ncl" in

# Module: k8s.io.v2

# PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
{
  metric | metricIdentifier | doc "metric identifies the target metric by name and selector",
  target | metricTarget | doc "target specifies the target value for the given metric"
}