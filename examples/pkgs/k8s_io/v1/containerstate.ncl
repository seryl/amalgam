# Module: k8s.io.v1.containerstate

let containerstatewaiting = import "./containerstatewaiting.ncl" in
let containerstateterminated = import "./containerstateterminated.ncl" in
let containerstaterunning = import "./containerstaterunning.ncl" in

{
  # ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
  ContainerState = {
    running | optional | containerstaterunning.ContainerStateRunning | doc "Details about a running container",
    terminated | optional | containerstateterminated.ContainerStateTerminated | doc "Details about a terminated container",
    waiting | optional | containerstatewaiting.ContainerStateWaiting | doc "Details about a waiting container",
  },
}
