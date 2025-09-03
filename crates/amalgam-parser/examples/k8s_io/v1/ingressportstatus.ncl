# IngressPortStatus represents the error condition of a service port
{
  error | optional | String | doc m%"
error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
  CamelCase names
- cloud provider specific error values must have names that comply with the
  format foo.example.com/CamelCase.
"%,
  port | optional | Number | doc "port is the port number of the ingress port.",
  protocol | optional | String | doc m%"
protocol is the protocol of the ingress port. The supported values are: "TCP", "UDP", "SCTP"
"%,
}
