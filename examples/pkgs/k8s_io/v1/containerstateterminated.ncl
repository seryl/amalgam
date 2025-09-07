# Module: k8s.io.v1

# ContainerStateTerminated is a terminated state of a container.
{
  containerID | optional | String | doc "Container's ID in the format '<type>://<container_id>'",
  exitCode | Number | doc "Exit status from the last termination of the container",
  finishedAt | optional | String | doc "Time at which the container last terminated",
  message | optional | String | doc "Message regarding the last termination of the container",
  reason | optional | String | doc "(brief) reason from the last termination of the container",
  signal | optional | Number | doc "Signal from the last termination of the container",
  startedAt | optional | String | doc "Time at which previous execution of the container started"
}