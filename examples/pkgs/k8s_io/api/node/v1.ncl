# Module: k8s.io.node.v1

let corev1 = import "../core/v1/mod.ncl" in
let toleration = corev1.Toleration in
let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let objectMeta = metav1.ObjectMeta in
let listMeta = metav1.ListMeta in

{
  Overhead = {
    podFixed
       | { .. }
       | doc "podFixed represents the fixed resource overhead associated with running a pod."
       | optional
  },

  RuntimeClass = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    handler
       | String
       | doc m%"
handler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called "runc" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.
"%,
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
    overhead
       | Overhead
       | doc m%"
overhead represents the resource overhead associated with running a pod for a given RuntimeClass. For more details, see
 https://kubernetes.io/docs/concepts/scheduling-eviction/pod-overhead/
"%
       | optional,
    scheduling
       | Scheduling
       | doc m%"
scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it. If scheduling is nil, this RuntimeClass is assumed to be supported by all nodes.
"%
       | optional
  },

  RuntimeClass = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array RuntimeClass
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

  Scheduling = {
    nodeSelector
       | { .. }
       | doc m%"
nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.
"%
       | optional,
    tolerations
       | Array toleration
       | doc m%"
tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.
"%
       | optional
  }
}
