let statusDetails = import "./StatusDetails.ncl" in
let listMeta = import "./ListMeta.ncl" in

# Module: k8s.io.v1

# Status is a return value for calls that don't return other objects.
{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  code
     | Number
     | doc "Suggested HTTP return code for this status, 0 if not set."
     | optional,
  details
     | statusDetails
     | doc m%"
Extended data associated with the reason.  Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  message
     | String
     | doc "A human-readable description of the status of this operation."
     | optional,
  metadata
     | listMeta
     | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  reason
     | String
     | doc m%"
A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
"%
     | optional,
  status
     | String
     | doc m%"
Status of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
     | optional
}