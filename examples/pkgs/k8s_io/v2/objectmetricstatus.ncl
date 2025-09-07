let metricValueStatus = import "./MetricValueStatus.ncl" in
let crossVersionObjectReference = import "./CrossVersionObjectReference.ncl" in
let metricIdentifier = import "./MetricIdentifier.ncl" in

# Module: k8s.io.v2

# ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
{
  current
     | metricValueStatus
     | doc "current contains the current value for the given metric",
  describedObject
     | crossVersionObjectReference
     | doc m%"
DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
  metric
     | metricIdentifier
     | doc "metric identifies the target metric by name and selector"
}