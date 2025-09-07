let ingressLoadBalancerStatus = import "./IngressLoadBalancerStatus.ncl" in

# Module: k8s.io.v1

# IngressStatus describe the current state of the Ingress.
{
  loadBalancer
     | ingressLoadBalancerStatus
     | doc "loadBalancer contains the current status of the load-balancer."
     | optional
}