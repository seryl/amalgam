# Module: k8s.io.v1

# ContainerStateWaiting is a waiting state of a container.
{
  message
     | String
     | doc "Message regarding why the container is not yet running."
     | optional,
  reason
     | String
     | doc "(brief) reason the container is not yet running."
     | optional
}