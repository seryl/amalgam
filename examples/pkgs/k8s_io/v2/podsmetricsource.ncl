# Module: k8s.io.v2.podsmetricsource

let metricidentifier = import "./metricidentifier.ncl" in
let metrictarget = import "./metrictarget.ncl" in

{
  # PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
  PodsMetricSource = {
    metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector",
    target | optional | metrictarget.MetricTarget | doc "target specifies the target value for the given metric",
  },
}
