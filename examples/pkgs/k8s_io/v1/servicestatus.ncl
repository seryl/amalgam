let condition = import "./Condition.ncl" in
let loadBalancerStatus = import "./LoadBalancerStatus.ncl" in

# Module: k8s.io.v1

# ServiceStatus represents the current status of a service.
{
  conditions
     | Array condition
     | doc "Current service state"
     | optional,
  loadBalancer
     | loadBalancerStatus
     | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%
     | optional
}