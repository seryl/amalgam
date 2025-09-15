# Module: k8s.io.flowcontrol.v1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let listMeta = metav1.ListMeta in
let time = metav1.Time in
let objectMeta = metav1.ObjectMeta in

{
  ExemptPriorityLevelConfiguration = {
    lendablePercent
       | Number
       | doc m%"
`lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels.  This value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.

LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
"%
       | optional,
    nominalConcurrencyShares
       | Number
       | doc m%"
`nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats nominally reserved for this priority level. This DOES NOT limit the dispatching from this priority level but affects the other priority levels through the borrowing mechanism. The server's concurrency limit (ServerCL) is divided among all the priority levels in proportion to their NCS values:

NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum[priority level k] NCS(k)

Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level. This field has a default value of zero.
"%
       | optional
  },

  FlowDistinguisherMethod = {
    type_field
       | String
       | doc m%"
`type` is the type of flow distinguisher method The supported types are "ByUser" and "ByNamespace". Required.
"%
  },

  FlowSchema = {
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
`metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | FlowSchemaSpec
       | doc m%"
`spec` is the specification of the desired behavior of a FlowSchema. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | FlowSchemaStatus
       | doc m%"
`status` is the current status of a FlowSchema. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  FlowSchemaCondition = {
    lastTransitionTime
       | time
       | doc m%"
`lastTransitionTime` is the last time the condition transitioned from one status to another.
"%
       | optional,
    message
       | String
       | doc "`message` is a human-readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc m%"
`reason` is a unique, one-word, CamelCase reason for the condition's last transition.
"%
       | optional,
    status
       | String
       | doc "`status` is the status of the condition. Can be True, False, Unknown. Required."
       | optional,
    type_field
       | String
       | doc "`type` is the type of the condition. Required."
       | optional
  },

  FlowSchema = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array FlowSchema
       | doc "`items` is a list of FlowSchemas.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc m%"
`metadata` is the standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  FlowSchemaSpec = {
    distinguisherMethod
       | FlowDistinguisherMethod
       | doc m%"
`distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema. `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
"%
       | optional,
    matchingPrecedence
       | Number
       | doc m%"
`matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen FlowSchema is among those with the numerically lowest (which we take to be logically highest) MatchingPrecedence.  Each MatchingPrecedence value must be ranged in [1,10000]. Note that if the precedence is not specified, it will be set to 1000 as default.
"%
       | optional,
    priorityLevelConfiguration
       | PriorityLevelConfigurationReference
       | doc m%"
`priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot be resolved, the FlowSchema will be ignored and marked as invalid in its status. Required.
"%,
    rules
       | Array PolicyRulesWithSubjects
       | doc m%"
`rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if at least one member of rules matches the request. if it is an empty slice, there will be no requests matching the FlowSchema.
"%
       | optional
  },

  FlowSchemaStatus = {
    conditions
       | Array FlowSchemaCondition
       | doc "`conditions` is a list of the current states of FlowSchema."
       | optional
  },

  GroupSubject = {
    name
       | String
       | doc m%"
name is the user group that matches, or "*" to match all user groups. See https://github.com/kubernetes/apiserver/blob/master/pkg/authentication/user/user.go for some well-known group names. Required.
"%
  },

  LimitResponse = {
    queuing
       | QueuingConfiguration
       | doc m%"
`queuing` holds the configuration parameters for queuing. This field may be non-empty only if `type` is `"Queue"`.
"%
       | optional,
    type_field
       | String
       | doc m%"
`type` is "Queue" or "Reject". "Queue" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. "Reject" means that requests that can not be executed upon arrival are rejected. Required.
"%
  },

  LimitedPriorityLevelConfiguration = {
    borrowingLimitPercent
       | Number
       | doc m%"
`borrowingLimitPercent`, if present, configures a limit on how many seats this priority level can borrow from other priority levels. The limit is known as this level's BorrowingConcurrencyLimit (BorrowingCL) and is a limit on the total number of seats that this level may borrow at any one time. This field holds the ratio of that limit to the level's nominal concurrency limit. When this field is non-nil, it must hold a non-negative integer and the limit is calculated as follows.

BorrowingCL(i) = round( NominalCL(i) * borrowingLimitPercent(i)/100.0 )

The value of this field can be more than 100, implying that this priority level can borrow a number of seats that is greater than its own nominal concurrency limit (NominalCL). When this field is left `nil`, the limit is effectively infinite.
"%
       | optional,
    lendablePercent
       | Number
       | doc m%"
`lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels. The value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.

LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
"%
       | optional,
    limitResponse
       | LimitResponse
       | doc m%"
`limitResponse` indicates what to do with requests that can not be executed right now
"%
       | optional,
    nominalConcurrencyShares
       | Number
       | doc m%"
`nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats available at this priority level. This is used both for requests dispatched from this priority level as well as requests dispatched from other priority levels borrowing seats from this level. The server's concurrency limit (ServerCL) is divided among the Limited priority levels in proportion to their NCS values:

NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum[priority level k] NCS(k)

Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level.

If not specified, this field defaults to a value of 30.

Setting this field to zero supports the construction of a "jail" for this priority level that is used to hold some request(s)
"%
       | optional
  },

  NonResourcePolicyRule = {
    nonResourceURLs
       | Array String
       | doc m%"
`nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:
  - "/healthz" is legal
  - "/hea*" is illegal
  - "/hea" is legal but matches nothing
  - "/hea/*" also matches nothing
  - "/healthz/*" matches all per-component health checks.
"*" matches all non-resource urls. if it is present, it must be the only entry. Required.
"%,
    verbs
       | Array String
       | doc m%"
`verbs` is a list of matching verbs and may not be empty. "*" matches all verbs. If it is present, it must be the only entry. Required.
"%
  },

  PolicyRulesWithSubjects = {
    nonResourceRules
       | Array NonResourcePolicyRule
       | doc m%"
`nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.
"%
       | optional,
    resourceRules
       | Array ResourcePolicyRule
       | doc m%"
`resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
"%
       | optional,
    subjects
       | Array Subject
       | doc m%"
subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.
"%
  },

  PriorityLevelConfiguration = {
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
`metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | PriorityLevelConfigurationSpec
       | doc m%"
`spec` is the specification of the desired behavior of a "request-priority". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | PriorityLevelConfigurationStatus
       | doc m%"
`status` is the current status of a "request-priority". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  PriorityLevelConfigurationCondition = {
    lastTransitionTime
       | time
       | doc m%"
`lastTransitionTime` is the last time the condition transitioned from one status to another.
"%
       | optional,
    message
       | String
       | doc "`message` is a human-readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc m%"
`reason` is a unique, one-word, CamelCase reason for the condition's last transition.
"%
       | optional,
    status
       | String
       | doc "`status` is the status of the condition. Can be True, False, Unknown. Required."
       | optional,
    type_field
       | String
       | doc "`type` is the type of the condition. Required."
       | optional
  },

  PriorityLevelConfiguration = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array PriorityLevelConfiguration
       | doc "`items` is a list of request-priorities.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc m%"
`metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  PriorityLevelConfigurationReference = {
    name
       | String
       | doc m%"
`name` is the name of the priority level configuration being referenced Required.
"%
  },

  PriorityLevelConfigurationSpec = {
    exempt
       | ExemptPriorityLevelConfiguration
       | doc m%"
`exempt` specifies how requests are handled for an exempt priority level. This field MUST be empty if `type` is `"Limited"`. This field MAY be non-empty if `type` is `"Exempt"`. If empty and `type` is `"Exempt"` then the default values for `ExemptPriorityLevelConfiguration` apply.
"%
       | optional,
    limited
       | LimitedPriorityLevelConfiguration
       | doc m%"
`limited` specifies how requests are handled for a Limited priority level. This field must be non-empty if and only if `type` is `"Limited"`.
"%
       | optional,
    type_field
       | String
       | doc m%"
`type` indicates whether this priority level is subject to limitation on request execution.  A value of `"Exempt"` means that requests of this priority level are not subject to a limit (and thus are never queued) and do not detract from the capacity made available to other priority levels.  A value of `"Limited"` means that (a) requests of this priority level _are_ subject to limits and (b) some of the server's limited capacity is made available exclusively to this priority level. Required.
"%
  },

  PriorityLevelConfigurationStatus = {
    conditions
       | Array PriorityLevelConfigurationCondition
       | doc "`conditions` is the current state of \"request-priority\"."
       | optional
  },

  QueuingConfiguration = {
    handSize
       | Number
       | doc m%"
`handSize` is a small positive number that configures the shuffle sharding of requests into queues.  When enqueuing a request at this priority level the request's flow identifier (a string pair) is hashed and the hash value is used to shuffle the list of queues and deal a hand of the size specified here.  The request is put into one of the shortest queues in that hand. `handSize` must be no larger than `queues`, and should be significantly smaller (so that a few heavy flows do not saturate most of the queues).  See the user-facing documentation for more extensive guidance on setting this field.  This field has a default value of 8.
"%
       | optional,
    queueLengthLimit
       | Number
       | doc m%"
`queueLengthLimit` is the maximum number of requests allowed to be waiting in a given queue of this priority level at a time; excess requests are rejected.  This value must be positive.  If not specified, it will be defaulted to 50.
"%
       | optional,
    queues
       | Number
       | doc m%"
`queues` is the number of queues for this priority level. The queues exist independently at each apiserver. The value must be positive.  Setting it to 1 effectively precludes shufflesharding and thus makes the distinguisher method of associated flow schemas irrelevant.  This field has a default value of 64.
"%
       | optional
  },

  ResourcePolicyRule = {
    apiGroups
       | Array String
       | doc m%"
`apiGroups` is a list of matching API groups and may not be empty. "*" matches all API groups and, if present, must be the only entry. Required.
"%,
    clusterScope
       | Bool
       | doc m%"
`clusterScope` indicates whether to match requests that do not specify a namespace (which happens either because the resource is not namespaced or the request targets all namespaces). If this field is omitted or false then the `namespaces` field must contain a non-empty list.
"%
       | optional,
    namespaces
       | Array String
       | doc m%"
`namespaces` is a list of target namespaces that restricts matches.  A request that specifies a target namespace matches only if either (a) this list contains that target namespace or (b) this list contains "*".  Note that "*" matches any specified namespace but does not match a request that _does not specify_ a namespace (see the `clusterScope` field for that). This list may be empty, but only if `clusterScope` is true.
"%
       | optional,
    resources
       | Array String
       | doc m%"
`resources` is a list of matching resources (i.e., lowercase and plural) with, if desired, subresource.  For example, [ "services", "nodes/status" ].  This list may not be empty. "*" matches all resources and, if present, must be the only entry. Required.
"%,
    verbs
       | Array String
       | doc m%"
`verbs` is a list of matching verbs and may not be empty. "*" matches all verbs and, if present, must be the only entry. Required.
"%
  },

  ServiceAccountSubject = {
    name
       | String
       | doc m%"
`name` is the name of matching ServiceAccount objects, or "*" to match regardless of name. Required.
"%,
    namespace
       | String
       | doc "`namespace` is the namespace of matching ServiceAccount objects. Required."
  },

  Subject = {
    group
       | GroupSubject
       | doc "`group` matches based on user group name."
       | optional,
    kind
       | String
       | doc "`kind` indicates which one of the other fields is non-empty. Required",
    serviceAccount
       | ServiceAccountSubject
       | doc "`serviceAccount` matches ServiceAccounts."
       | optional,
    user
       | UserSubject
       | doc "`user` matches based on username."
       | optional
  },

  UserSubject = {
    name
       | String
       | doc "`name` is the username that matches, or \"*\" to match all usernames. Required."
  }
}
