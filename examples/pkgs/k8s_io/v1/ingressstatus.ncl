let ingressloadbalancerstatus = import "./ingressloadbalancerstatus.ncl" in

# Module: k8s.io.v1

# IngressStatus describe the current state of the Ingress.
{
  loadBalancer | optional | ingressloadbalancerstatus | doc "loadBalancer contains the current status of the load-balancer."
}