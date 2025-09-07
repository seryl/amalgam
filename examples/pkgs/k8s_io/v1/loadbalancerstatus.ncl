let loadBalancerIngress = import "./LoadBalancerIngress.ncl" in

# Module: k8s.io.v1

# LoadBalancerStatus represents the status of a load-balancer.
{
  ingress
     | Array loadBalancerIngress
     | doc m%"
Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
"%
     | optional
}