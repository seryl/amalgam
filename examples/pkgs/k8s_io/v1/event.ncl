let objectreference = import "./objectreference.ncl" in
let objectmeta = import "./objectmeta.ncl" in
let eventseries = import "./eventseries.ncl" in
let eventsource = import "./eventsource.ncl" in

# Module: k8s.io.v1

# Event is a report of an event somewhere in the cluster.  Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.
{
  action | optional | String | doc "What action was taken/failed regarding to the Regarding object.",
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  count | optional | Number | doc "The number of times this event has occurred.",
  eventTime | optional | String | doc "Time when this Event was first observed.",
  firstTimestamp | optional | String | doc m%"
The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
"%,
  involvedObject | optional | objectreference | doc "The object that this event is about.",
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  lastTimestamp | optional | String | doc "The time at which the most recent occurrence of this event was recorded.",
  message | optional | String | doc "A human-readable description of the status of this operation.",
  metadata | optional | objectmeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  reason | optional | String | doc m%"
This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
"%,
  related | optional | objectreference | doc "Optional secondary object for more complex actions.",
  reportingComponent | optional | String | doc "Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.",
  reportingInstance | optional | String | doc "ID of the controller instance, e.g. `kubelet-xyzf`.",
  series | optional | eventseries | doc m%"
Data about the Event series this event represents or nil if it's a singleton Event.
"%,
  source | optional | eventsource | doc m%"
The component reporting this event. Should be a short machine understandable string.
"%,
  type | optional | String | doc "Type of this event (Normal, Warning), new types could be added in the future"
}