# Module: k8s.io.v1.ingressloadbalancerstatus

let ingressloadbalanceringress = import "./ingressloadbalanceringress.ncl" in

{
  # IngressLoadBalancerStatus represents the status of a load-balancer.
  IngressLoadBalancerStatus = {
    ingress | optional | Array ingressloadbalanceringress.IngressLoadBalancerIngress | doc "ingress is a list containing ingress points for the load-balancer.",
  },
}
