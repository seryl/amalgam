# Module: k8s.io.v1

# ContainerStateTerminated is a terminated state of a container.
{
  containerID
     | String
     | doc "Container's ID in the format '<type>://<container_id>'"
     | optional,
  exitCode
     | Number
     | doc "Exit status from the last termination of the container",
  finishedAt
     | String
     | doc "Time at which the container last terminated"
     | optional,
  message
     | String
     | doc "Message regarding the last termination of the container"
     | optional,
  reason
     | String
     | doc "(brief) reason from the last termination of the container"
     | optional,
  signal
     | Number
     | doc "Signal from the last termination of the container"
     | optional,
  startedAt
     | String
     | doc "Time at which previous execution of the container started"
     | optional
}