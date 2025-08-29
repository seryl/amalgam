# Module: k8s.io.v1.lifecyclehandler

let tcpsocketaction = import "./tcpsocketaction.ncl" in
let httpgetaction = import "./httpgetaction.ncl" in
let execaction = import "./execaction.ncl" in
let sleepaction = import "./sleepaction.ncl" in

{
  # LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
  LifecycleHandler = {
    exec | optional | execaction.ExecAction | doc "Exec specifies the action to take.",
    httpGet | optional | httpgetaction.HTTPGetAction | doc "HTTPGet specifies the http request to perform.",
    sleep | optional | sleepaction.SleepAction | doc m%"
Sleep represents the duration that the container should sleep before being terminated.
"%,
    tcpSocket | optional | tcpsocketaction.TCPSocketAction | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
"%,
  },
}
