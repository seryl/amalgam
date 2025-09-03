# Module: k8s.io.v2

# ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
{
  current | optional | metricvaluestatus.MetricValueStatus | doc "current contains the current value for the given metric",
  describedObject | optional | crossversionobjectreference.CrossVersionObjectReference | doc m%"
DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
  metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector"
}
