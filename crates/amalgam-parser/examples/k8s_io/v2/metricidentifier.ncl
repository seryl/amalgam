let labelselector = import "../v1/labelselector.ncl" in

# MetricIdentifier defines the name and optionally selector for a metric
{
  name | optional | String | doc "name is the name of the given metric",
  selector | optional | labelselector.LabelSelector | doc m%"
selector is the string-encoded form of a standard kubernetes label selector for the given metric When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping. When unset, just the metricName will be used to gather metrics.
"%,
}
