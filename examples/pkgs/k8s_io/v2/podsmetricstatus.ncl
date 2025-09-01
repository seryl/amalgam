# Module: k8s.io.v2.podsmetricstatus

let metricvaluestatus = import "./metricvaluestatus.ncl" in
let metricidentifier = import "./metricidentifier.ncl" in

{
  # PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
  PodsMetricStatus = {
    current | optional | metricvaluestatus.MetricValueStatus | doc "current contains the current value for the given metric",
    metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector",
  },
}
