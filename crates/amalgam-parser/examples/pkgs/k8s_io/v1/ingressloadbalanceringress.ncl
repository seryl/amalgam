# Module: k8s.io.v1

# IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
{
  hostname | optional | String | doc "hostname is set for load-balancer ingress points that are DNS based.",
  ip | optional | String | doc "ip is set for load-balancer ingress points that are IP based.",
  ports | optional | Array ingressportstatus.IngressPortStatus | doc "ports provides information about the ports exposed by this LoadBalancer."
}
