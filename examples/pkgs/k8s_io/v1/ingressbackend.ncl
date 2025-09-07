let typedLocalObjectReference = import "./TypedLocalObjectReference.ncl" in
let ingressServiceBackend = import "./IngressServiceBackend.ncl" in

# Module: k8s.io.v1

# IngressBackend describes all endpoints for a given service and port.
{
  resource
     | typedLocalObjectReference
     | doc m%"
resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a service.Name and service.Port must not be specified. This is a mutually exclusive setting with "Service".
"%
     | optional,
  service
     | ingressServiceBackend
     | doc m%"
service references a service as a backend. This is a mutually exclusive setting with "Resource".
"%
     | optional
}