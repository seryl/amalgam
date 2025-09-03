# Module: k8s.io.coordination.v1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let microTime = metav1.MicroTime in
let listMeta = metav1.ListMeta in
let objectMeta = metav1.ObjectMeta in

{
  Lease = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc m%"
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | LeaseSpec
       | doc m%"
spec contains the specification of the Lease. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  Lease = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Lease
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

  LeaseSpec = {
    acquireTime
       | microTime
       | doc "acquireTime is a time when the current lease was acquired."
       | optional,
    holderIdentity
       | String
       | doc m%"
holderIdentity contains the identity of the holder of a current lease. If Coordinated Leader Election is used, the holder identity must be equal to the elected LeaseCandidate.metadata.name field.
"%
       | optional,
    leaseDurationSeconds
       | Number
       | doc m%"
leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measured against the time of last observed renewTime.
"%
       | optional,
    leaseTransitions
       | Number
       | doc "leaseTransitions is the number of transitions of a lease between holders."
       | optional,
    preferredHolder
       | String
       | doc m%"
PreferredHolder signals to a lease holder that the lease has a more optimal holder and should be given up. This field can only be set if Strategy is also set.
"%
       | optional,
    renewTime
       | microTime
       | doc m%"
renewTime is a time when the current holder of a lease has last updated the lease.
"%
       | optional,
    strategy
       | String
       | doc m%"
Strategy indicates the strategy for picking the leader for coordinated leader election. If the field is not specified, there is no active coordination for this lease. (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
"%
       | optional
  }
}
