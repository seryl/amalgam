# ContainerStateWaiting is a waiting state of a container.
{
  message | optional | String | doc "Message regarding why the container is not yet running.",
  reason | optional | String | doc "(brief) reason the container is not yet running.",
}
