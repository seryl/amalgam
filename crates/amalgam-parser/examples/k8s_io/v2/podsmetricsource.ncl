# PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
{
  metric | optional | MetricIdentifier | doc "metric identifies the target metric by name and selector",
  target | optional | MetricTarget | doc "target specifies the target value for the given metric",
}
