# Module: k8s.io.v2

# MetricValueStatus holds the current value for a metric
{
  averageUtilization
     | Number
     | doc m%"
currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
"%
     | optional,
  averageValue
     | String
     | doc m%"
averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
"%
     | optional,
  value
     | String
     | doc "value is the current value of the metric (as a quantity)."
     | optional
}