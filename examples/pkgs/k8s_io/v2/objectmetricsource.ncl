let crossVersionObjectReference = import "./CrossVersionObjectReference.ncl" in
let metricIdentifier = import "./MetricIdentifier.ncl" in
let metricTarget = import "./MetricTarget.ncl" in

# Module: k8s.io.v2

# ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
{
  describedObject | crossVersionObjectReference | doc m%"
describedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
  metric | metricIdentifier | doc "metric identifies the target metric by name and selector",
  target | metricTarget | doc "target specifies the target value for the given metric"
}