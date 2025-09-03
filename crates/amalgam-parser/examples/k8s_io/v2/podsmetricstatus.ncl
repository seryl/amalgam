# PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
{
  current | optional | MetricValueStatus | doc "current contains the current value for the given metric",
  metric | optional | MetricIdentifier | doc "metric identifies the target metric by name and selector",
}
