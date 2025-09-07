let hTTPHeader = import "./HTTPHeader.ncl" in

# Module: k8s.io.v1

# HTTPGetAction describes an action based on HTTP Get requests.
{
  host
     | String
     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
"%
     | optional,
  httpHeaders
     | Array hTTPHeader
     | doc "Custom headers to set in the request. HTTP allows repeated headers."
     | optional,
  path
     | String
     | doc "Path to access on the HTTP server."
     | optional,
  port
     | std.contract.from_predicate (fun value => std.is_number value || std.is_string value)
     | doc m%"
Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
"%,
  scheme
     | String
     | doc "Scheme to use for connecting to the host. Defaults to HTTP."
     | optional
}