# Module: k8s.io.v1

# ContainerPort represents a network port in a single container.
{
  containerPort | Number | doc m%"
Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
"%,
  hostIP | optional | String | doc "What host IP to bind the external port to.",
  hostPort | optional | Number | doc m%"
Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
"%,
  name | optional | String | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
"%,
  protocol | optional | String | doc "Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\"."
}