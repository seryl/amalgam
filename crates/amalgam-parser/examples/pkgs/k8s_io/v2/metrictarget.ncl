# Module: k8s.io.v2

# MetricTarget defines the target value, average value, or average utilization of a specific metric
{
  averageUtilization | optional | Number | doc m%"
averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
"%,
  averageValue | optional | String | doc m%"
averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
"%,
  type | optional | String | doc "type represents whether the metric type is Utilization, Value, or AverageValue",
  value | optional | String | doc "value is the target value of the metric (as a quantity)."
}
