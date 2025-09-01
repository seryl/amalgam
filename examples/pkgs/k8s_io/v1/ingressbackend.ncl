# Module: k8s.io.v1.ingressbackend

let typedlocalobjectreference = import "./typedlocalobjectreference.ncl" in
let ingressservicebackend = import "./ingressservicebackend.ncl" in

{
  # IngressBackend describes all endpoints for a given service and port.
  IngressBackend = {
    resource | optional | typedlocalobjectreference.TypedLocalObjectReference | doc m%"
resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a service.Name and service.Port must not be specified. This is a mutually exclusive setting with "Service".
"%,
    service | optional | ingressservicebackend.IngressServiceBackend | doc m%"
service references a service as a backend. This is a mutually exclusive setting with "Resource".
"%,
  },
}
