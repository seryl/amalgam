let containerStateRunning = import "./ContainerStateRunning.ncl" in
let containerStateTerminated = import "./ContainerStateTerminated.ncl" in
let containerStateWaiting = import "./ContainerStateWaiting.ncl" in

# Module: k8s.io.v1

# ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
{
  running
     | containerStateRunning
     | doc "Details about a running container"
     | optional,
  terminated
     | containerStateTerminated
     | doc "Details about a terminated container"
     | optional,
  waiting
     | containerStateWaiting
     | doc "Details about a waiting container"
     | optional
}