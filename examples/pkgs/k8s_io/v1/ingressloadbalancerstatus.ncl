let ingressLoadBalancerIngress = import "./IngressLoadBalancerIngress.ncl" in

# Module: k8s.io.v1

# IngressLoadBalancerStatus represents the status of a load-balancer.
{
  ingress
     | Array ingressLoadBalancerIngress
     | doc "ingress is a list containing ingress points for the load-balancer."
     | optional
}