# Module: io.k8s.kube-aggregator.pkg.apis.apiregistration.v1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let listMeta = metav1.ListMeta in
let objectMeta = metav1.ObjectMeta in
let time = metav1.Time in

{
  APIService = {
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
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | APIServiceSpec
       | doc "Spec contains information for locating and communicating with a server"
       | optional,
    status
       | APIServiceStatus
       | doc "Status contains derived information about an API server"
       | optional
  },

  APIServiceCondition = {
    lastTransitionTime
       | time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "Human-readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc "Unique, one-word, CamelCase reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status is the status of the condition. Can be True, False, Unknown.",
    type_field
       | String
       | doc "Type is the type of the condition."
  },

  APIService = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array APIService
       | doc "Items is the list of APIService",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | listMeta
       | doc m%"
Standard list metadata More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  APIServiceSpec = {
    caBundle
       | String
       | doc m%"
CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate. If unspecified, system trust roots on the apiserver are used.
"%
       | optional,
    group
       | String
       | doc "Group is the API group name this server hosts"
       | optional,
    groupPriorityMinimum
       | Number
       | doc m%"
GroupPriorityMinimum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMinimum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) We'd recommend something like: *.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s
"%,
    insecureSkipTLSVerify
       | Bool
       | doc m%"
InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged.  You should use the CABundle instead.
"%
       | optional,
    service
       | ServiceReference
       | doc m%"
Service is a reference to the service for this API server.  It must communicate on port 443. If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
"%
       | optional,
    version
       | String
       | doc "Version is the API version this server hosts.  For example, \"v1\""
       | optional,
    versionPriority
       | Number
       | doc m%"
VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). Since it's inside of a group, the number can be small, probably in the 10s. In case of equal version priorities, the version string will be used to compute the order inside a group. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
"%
  },

  APIServiceStatus = {
    conditions
       | Array APIServiceCondition
       | doc "Current service state of apiService."
       | optional
  },

  ServiceReference = {
    name
       | String
       | doc "Name is the name of the service"
       | optional,
    namespace
       | String
       | doc "Namespace is the namespace of the service"
       | optional,
    port
       | Number
       | doc m%"
If specified, the port on the service that hosting webhook. Default to 443 for backward compatibility. `port` should be a valid port number (1-65535, inclusive).
"%
       | optional
  }
}
