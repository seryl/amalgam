let execAction = import "./ExecAction.ncl" in
let hTTPGetAction = import "./HTTPGetAction.ncl" in
let sleepAction = import "./SleepAction.ncl" in
let tCPSocketAction = import "./TCPSocketAction.ncl" in

# Module: k8s.io.v1

# LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
{
  exec | optional | execAction | doc "Exec specifies a command to execute in the container.",
  httpGet | optional | hTTPGetAction | doc "HTTPGet specifies an HTTP GET request to perform.",
  sleep | optional | sleepAction | doc "Sleep represents a duration that the container should sleep.",
  tcpSocket | optional | tCPSocketAction | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for backward compatibility. There is no validation of this field and lifecycle hooks will fail at runtime when it is specified.
"%
}