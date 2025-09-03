# IngressBackend describes all endpoints for a given service and port.
{
  resource | optional | TypedLocalObjectReference | doc m%"
resource is an ObjectRef to another Kubernetes resource in the namespace of the Ingress object. If resource is specified, a service.Name and service.Port must not be specified. This is a mutually exclusive setting with "Service".
"%,
  service | optional | IngressServiceBackend | doc m%"
service references a service as a backend. This is a mutually exclusive setting with "Resource".
"%,
}
