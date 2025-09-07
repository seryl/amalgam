let objectReference = import "./ObjectReference.ncl" in

# Module: k8s.io.v1

# EndpointAddress is a tuple that describes single IP address. Deprecated: This API is deprecated in v1.33+.
{
  hostname | optional | String | doc "The Hostname of this endpoint",
  ip | String | doc m%"
The IP of this endpoint. May not be loopback (127.0.0.0/8 or ::1), link-local (169.254.0.0/16 or fe80::/10), or link-local multicast (224.0.0.0/24 or ff02::/16).
"%,
  nodeName | optional | String | doc m%"
Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
"%,
  targetRef | optional | objectReference | doc "Reference to object providing the endpoint."
}