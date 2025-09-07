let hPAScalingRules = import "./HPAScalingRules.ncl" in

# Module: k8s.io.v2

# HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
{
  scaleDown
     | hPAScalingRules
     | doc m%"
scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
"%
     | optional,
  scaleUp
     | hPAScalingRules
     | doc m%"
scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
  * increase no more than 4 pods per 60 seconds
  * double the number of pods per 60 seconds
No stabilization is used.
"%
     | optional
}