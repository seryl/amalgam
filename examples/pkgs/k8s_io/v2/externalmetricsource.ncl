let metricIdentifier = import "./MetricIdentifier.ncl" in
let metricTarget = import "./MetricTarget.ncl" in

# Module: k8s.io.v2

# ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
{
  metric
     | metricIdentifier
     | doc "metric identifies the target metric by name and selector",
  target
     | metricTarget
     | doc "target specifies the target value for the given metric"
}