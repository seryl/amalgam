# Module: k8s.io.v1.eventsource

{
  # EventSource contains information for an event.
  EventSource = {
    component | optional | String | doc "Component from which the event is generated.",
    host | optional | String | doc "Node name on which the event is generated.",
  },
}
