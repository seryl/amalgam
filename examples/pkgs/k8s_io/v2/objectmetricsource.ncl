# Module: k8s.io.v2.objectmetricsource

let metrictarget = import "./metrictarget.ncl" in
let crossversionobjectreference = import "./crossversionobjectreference.ncl" in
let metricidentifier = import "./metricidentifier.ncl" in

{
  # ObjectMetricSource indicates how to scale on a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
  ObjectMetricSource = {
    describedObject | optional | crossversionobjectreference.CrossVersionObjectReference | doc m%"
describedObject specifies the descriptions of a object,such as kind,name apiVersion
"%,
    metric | optional | metricidentifier.MetricIdentifier | doc "metric identifies the target metric by name and selector",
    target | optional | metrictarget.MetricTarget | doc "target specifies the target value for the given metric",
  },
}
