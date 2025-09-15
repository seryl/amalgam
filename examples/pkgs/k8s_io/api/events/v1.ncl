# Module: k8s.io.events.v1

let corev1 = import "../core/v1/mod.ncl" in
let objectReference = corev1.ObjectReference in
let eventSource = corev1.EventSource in
let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let microTime = metav1.MicroTime in
let listMeta = metav1.ListMeta in
let objectMeta = metav1.ObjectMeta in
let time = metav1.Time in

{
  Event = {
    action
       | String
       | doc m%"
action is what action was taken/failed regarding to the regarding object. It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters.
"%
       | optional,
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    deprecatedCount
       | Number
       | doc m%"
deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    deprecatedFirstTimestamp
       | time
       | doc m%"
deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    deprecatedLastTimestamp
       | time
       | doc m%"
deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    deprecatedSource
       | eventSource
       | doc m%"
deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    eventTime
       | microTime
       | doc "eventTime is the time when this Event was first observed. It is required.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    note
       | String
       | doc m%"
note is a human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
"%
       | optional,
    reason
       | String
       | doc m%"
reason is why the action was taken. It is human-readable. This field cannot be empty for new Events and it can have at most 128 characters.
"%
       | optional,
    regarding
       | objectReference
       | doc m%"
regarding contains the object this Event is about. In most cases it's an Object reporting controller implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
"%
       | optional,
    related
       | objectReference
       | doc m%"
related is the optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
"%
       | optional,
    reportingController
       | String
       | doc m%"
reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events.
"%
       | optional,
    reportingInstance
       | String
       | doc m%"
reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`. This field cannot be empty for new Events and it can have at most 128 characters.
"%
       | optional,
    series
       | EventSeries
       | doc m%"
series is data about the Event series this event represents or nil if it's a singleton Event.
"%
       | optional,
    type_field
       | String
       | doc m%"
type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events.
"%
       | optional
  },

  Event = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Event
       | doc "items is a list of schema objects.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  EventSeries = {
    count
       | Number
       | doc "count is the number of occurrences in this series up to the last heartbeat time.",
    lastObservedTime
       | microTime
       | doc m%"
lastObservedTime is the time when last Event from the series was seen before last heartbeat.
"%
  }
}
