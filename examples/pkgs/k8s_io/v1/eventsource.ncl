# Module: k8s.io.v1

# EventSource contains information for an event.
{
  component
     | String
     | doc "Component from which the event is generated."
     | optional,
  host
     | String
     | doc "Node name on which the event is generated."
     | optional
}