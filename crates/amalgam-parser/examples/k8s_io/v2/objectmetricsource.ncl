# ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
{
  describedObject | optional | CrossVersionObjectReference | doc m%"
describedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
  metric | optional | MetricIdentifier | doc "metric identifies the target metric by name and selector",
  target | optional | MetricTarget | doc "target specifies the target value for the given metric",
}
