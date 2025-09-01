# Module: k8s.io.v1.ingressservicebackend

let servicebackendport = import "./servicebackendport.ncl" in

{
  # IngressServiceBackend references a Kubernetes Service as a Backend.
  IngressServiceBackend = {
    name | optional | String | doc m%"
name is the referenced service. The service must exist in the same namespace as the Ingress object.
"%,
    port | optional | servicebackendport.ServiceBackendPort | doc m%"
port of the referenced service. A port name or port number is required for a IngressServiceBackend.
"%,
  },
}
