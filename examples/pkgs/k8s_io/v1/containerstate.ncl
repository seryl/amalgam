let containerStateRunning = import "./ContainerStateRunning.ncl" in
let containerStateTerminated = import "./ContainerStateTerminated.ncl" in
let containerStateWaiting = import "./ContainerStateWaiting.ncl" in

# Module: k8s.io.v1

# ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
{
  running | optional | containerStateRunning | doc "Details about a running container",
  terminated | optional | containerStateTerminated | doc "Details about a terminated container",
  waiting | optional | containerStateWaiting | doc "Details about a waiting container"
}