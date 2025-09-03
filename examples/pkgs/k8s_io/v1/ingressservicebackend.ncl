let servicebackendport = import "./servicebackendport.ncl" in

# Module: k8s.io.v1

# IngressServiceBackend references a Kubernetes Service as a Backend.
{
  name | optional | String | doc m%"
name is the referenced service. The service must exist in the same namespace as the Ingress object.
"%,
  port | optional | servicebackendport | doc m%"
port of the referenced service. A port name or port number is required for a IngressServiceBackend.
"%
}