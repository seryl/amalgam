# HTTPGetAction describes an action based on HTTP Get requests.
{
  host | optional | String | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
"%,
  httpHeaders | optional | Array HTTPHeader | doc "Custom headers to set in the request. HTTP allows repeated headers.",
  path | optional | String | doc "Path to access on the HTTP server.",
  port | optional | String | doc m%"
Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
"%,
  scheme | optional | String | doc "Scheme to use for connecting to the host. Defaults to HTTP.",
}
