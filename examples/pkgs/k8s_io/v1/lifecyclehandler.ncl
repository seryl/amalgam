let execaction = import "./execaction.ncl" in
let httpgetaction = import "./httpgetaction.ncl" in
let sleepaction = import "./sleepaction.ncl" in
let tcpsocketaction = import "./tcpsocketaction.ncl" in

# Module: k8s.io.v1

# LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
{
  exec | optional | execaction | doc "Exec specifies a command to execute in the container.",
  httpGet | optional | httpgetaction | doc "HTTPGet specifies an HTTP GET request to perform.",
  sleep | optional | sleepaction | doc "Sleep represents a duration that the container should sleep.",
  tcpSocket | optional | tcpsocketaction | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for backward compatibility. There is no validation of this field and lifecycle hooks will fail at runtime when it is specified.
"%
}