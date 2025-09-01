# Module: k8s.io.v2.metricvaluestatus

{
  # MetricValueStatus holds the current value for a metric
  MetricValueStatus = {
    averageUtilization | optional | Number | doc m%"
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
"%,
    averageValue | optional | String | doc m%"
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
"%,
    value | optional | String | doc "value is the current value of the metric (as a quantity).",
  },
}
