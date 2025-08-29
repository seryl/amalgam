# Module: k8s.io.v1.ingressstatus

let ingressloadbalancerstatus = import "./ingressloadbalancerstatus.ncl" in

{
  # IngressStatus describe the current state of the Ingress.
  IngressStatus = {
    loadBalancer | optional | ingressloadbalancerstatus.IngressLoadBalancerStatus | doc "loadBalancer contains the current status of the load-balancer.",
  },
}
