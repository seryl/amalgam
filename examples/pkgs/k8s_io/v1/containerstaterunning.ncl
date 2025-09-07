# Module: k8s.io.v1

# ContainerStateRunning is a running state of a container.
{
  startedAt
     | String
     | doc "Time at which the container was last (re-)started"
     | optional
}