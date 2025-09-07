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
       | Time
       | doc m%"
deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    deprecatedLastTimestamp
       | Time
       | doc m%"
deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    deprecatedSource
       | EventSource
       | doc m%"
deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
"%
       | optional,
    eventTime
       | MicroTime
       | doc "eventTime is the time when this Event was first observed. It is required.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ObjectMeta
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
       | ObjectReference
       | doc m%"
regarding contains the object this Event is about. In most cases it's an Object reporting controller implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
"%
       | optional,
    related
       | ObjectReference
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
       | io.k8s.api.events.v1.EventSeries
       | doc m%"
series is data about the Event series this event represents or nil if it's a singleton Event.
"%
       | optional,
    type
       | String
       | doc m%"
type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events.
"%
       | optional
  },

  EventList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.events.v1.Event
       | doc "items is a list of schema objects.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
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
       | MicroTime
       | doc m%"
lastObservedTime is the time when last Event from the series was seen before last heartbeat.
"%
  }
}