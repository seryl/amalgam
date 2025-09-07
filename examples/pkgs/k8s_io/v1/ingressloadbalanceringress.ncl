let ingressPortStatus = import "./IngressPortStatus.ncl" in

# Module: k8s.io.v1

# IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
{
  hostname
     | String
     | doc "hostname is set for load-balancer ingress points that are DNS based."
     | optional,
  ip
     | String
     | doc "ip is set for load-balancer ingress points that are IP based."
     | optional,
  ports
     | Array ingressPortStatus
     | doc "ports provides information about the ports exposed by this LoadBalancer."
     | optional
}