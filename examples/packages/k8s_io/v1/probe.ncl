# Module: k8s.io.v1.probe

let tcpsocketaction = import "./tcpsocketaction.ncl" in
let grpcaction = import "./grpcaction.ncl" in
let httpgetaction = import "./httpgetaction.ncl" in
let execaction = import "./execaction.ncl" in

{
  # Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
  Probe = {
    exec | optional | execaction.ExecAction | doc "Exec specifies the action to take.",
    failureThreshold | optional | Number | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
"%,
    grpc | optional | grpcaction.GRPCAction | doc "GRPC specifies an action involving a GRPC port.",
    httpGet | optional | httpgetaction.HTTPGetAction | doc "HTTPGet specifies the http request to perform.",
    initialDelaySeconds | optional | Number | doc m%"
Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%,
    periodSeconds | optional | Number | doc m%"
How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
"%,
    successThreshold | optional | Number | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%,
    tcpSocket | optional | tcpsocketaction.TCPSocketAction | doc "TCPSocket specifies an action involving a TCP port.",
    terminationGracePeriodSeconds | optional | Number | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%,
    timeoutSeconds | optional | Number | doc m%"
Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%,
  },
}
