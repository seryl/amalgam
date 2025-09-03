# ServiceStatus represents the current status of a service.
{
  conditions | optional | Array Condition | doc "Current service state",
  loadBalancer | optional | LoadBalancerStatus | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%,
}
