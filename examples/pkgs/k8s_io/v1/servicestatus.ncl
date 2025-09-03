let condition = import "./condition.ncl" in
let loadbalancerstatus = import "./loadbalancerstatus.ncl" in

# Module: k8s.io.v1

# ServiceStatus represents the current status of a service.
{
  conditions | optional | Array condition | doc "Current service state",
  loadBalancer | optional | loadbalancerstatus | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%
}