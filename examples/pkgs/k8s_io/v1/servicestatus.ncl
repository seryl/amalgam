# Module: k8s.io.v1.servicestatus

let loadbalancerstatus = import "./loadbalancerstatus.ncl" in
let condition = import "./condition.ncl" in

{
  # ServiceStatus represents the current status of a service.
  ServiceStatus = {
    conditions | optional | Array condition.Condition | doc "Current service state",
    loadBalancer | optional | loadbalancerstatus.LoadBalancerStatus | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%,
  },
}
