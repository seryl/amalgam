# Module: k8s.io.v1.tcpsocketaction

{
  # TCPSocketAction describes an action based on opening a socket
  TCPSocketAction = {
    host | optional | String | doc "Optional: Host name to connect to, defaults to the pod IP.",
    port | Number | String | doc m%"
Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
"%,
  },
}
