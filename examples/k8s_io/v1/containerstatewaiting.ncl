# Module: k8s.io.v1.containerstatewaiting

{
  # ContainerStateWaiting is a waiting state of a container.
  ContainerStateWaiting = {
    message | optional | String | doc "Message regarding why the container is not yet running.",
    reason | optional | String | doc "(brief) reason the container is not yet running.",
  },
}
