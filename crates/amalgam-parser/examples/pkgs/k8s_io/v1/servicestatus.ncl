# Module: k8s.io.v1

# ServiceStatus represents the current status of a service.
{
  conditions | optional | Array condition.Condition | doc "Current service state",
  loadBalancer | optional | loadbalancerstatus.LoadBalancerStatus | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%
}
