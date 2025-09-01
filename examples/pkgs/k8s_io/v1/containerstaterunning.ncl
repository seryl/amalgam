# Module: k8s.io.v1.containerstaterunning

{
  # ContainerStateRunning is a running state of a container.
  ContainerStateRunning = {
    startedAt | optional | String | doc "Time at which the container was last (re-)started",
  },
}
