# Module: k8s.io.v2

# ExternalMetricSource indicates how to scale on a metric not associated with any Kubernetes object (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
{
  metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector",
  target | optional | metrictarget.MetricTarget | doc "target specifies the target value for the given metric"
}
