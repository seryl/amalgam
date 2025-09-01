# Module: k8s.io.v2.externalmetricstatus

let metricvaluestatus = import "./metricvaluestatus.ncl" in
let metricidentifier = import "./metricidentifier.ncl" in

{
  # ExternalMetricStatus indicates the current value of a global metric not associated with any Kubernetes object.
  ExternalMetricStatus = {
    current | optional | metricvaluestatus.MetricValueStatus | doc "current contains the current value for the given metric",
    metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector",
  },
}
