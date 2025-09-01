# Module: k8s.io.v1.loadbalancerstatus

let loadbalanceringress = import "./loadbalanceringress.ncl" in

{
  # LoadBalancerStatus represents the status of a load-balancer.
  LoadBalancerStatus = {
    ingress | optional | Array loadbalanceringress.LoadBalancerIngress | doc m%"
Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
"%,
  },
}
