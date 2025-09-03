# ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
{
  running | optional | ContainerStateRunning | doc "Details about a running container",
  terminated | optional | ContainerStateTerminated | doc "Details about a terminated container",
  waiting | optional | ContainerStateWaiting | doc "Details about a waiting container",
}
