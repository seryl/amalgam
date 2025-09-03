let ingressloadbalanceringress = import "./ingressloadbalanceringress.ncl" in

# Module: k8s.io.v1

# IngressLoadBalancerStatus represents the status of a load-balancer.
{
  ingress | optional | Array ingressloadbalanceringress | doc "ingress is a list containing ingress points for the load-balancer."
}