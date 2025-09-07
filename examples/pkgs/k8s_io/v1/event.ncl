let objectReference = import "./ObjectReference.ncl" in
let objectMeta = import "./ObjectMeta.ncl" in
let eventSeries = import "./EventSeries.ncl" in
let eventSource = import "./EventSource.ncl" in

# Module: k8s.io.v1

# Event is a report of an event somewhere in the cluster.  Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.
{
  action
     | String
     | doc "What action was taken/failed regarding to the Regarding object."
     | optional,
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  count
     | Number
     | doc "The number of times this event has occurred."
     | optional,
  eventTime
     | String
     | doc "Time when this Event was first observed."
     | optional,
  firstTimestamp
     | String
     | doc m%"
The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
"%
     | optional,
  involvedObject
     | objectReference
     | doc "The object that this event is about.",
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  lastTimestamp
     | String
     | doc "The time at which the most recent occurrence of this event was recorded."
     | optional,
  message
     | String
     | doc "A human-readable description of the status of this operation."
     | optional,
  metadata
     | objectMeta
     | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  reason
     | String
     | doc m%"
This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
"%
     | optional,
  related
     | objectReference
     | doc "Optional secondary object for more complex actions."
     | optional,
  reportingComponent
     | String
     | doc "Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`."
     | optional,
  reportingInstance
     | String
     | doc "ID of the controller instance, e.g. `kubelet-xyzf`."
     | optional,
  series
     | eventSeries
     | doc m%"
Data about the Event series this event represents or nil if it's a singleton Event.
"%
     | optional,
  source
     | eventSource
     | doc m%"
The component reporting this event. Should be a short machine understandable string.
"%
     | optional,
  type
     | String
     | doc "Type of this event (Normal, Warning), new types could be added in the future"
     | optional
}