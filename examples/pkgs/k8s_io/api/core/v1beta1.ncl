# Module: k8s.io.v1beta1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let condition = metav1.Condition in
let listMeta = metav1.ListMeta in
let objectMeta = metav1.ObjectMeta in

{
  IPAddress = {
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
      | IPAddressSpec
      | doc m%"
spec is the desired state of the IPAddress. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
      | optional
  },

  IPAddressList = {
    apiVersion
      | String
      | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
      | optional,
    items
      | Array IPAddress
      | doc "items is the list of IPAddresses.",
    kind
      | String
      | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
      | optional,
    metadata
      | listMeta
      | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
      | optional
  },

  IPAddressSpec = {
    parentRef
      | ParentReference
      | doc m%"
ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.
"%
  },

  ParentReference = {
    group
      | String
      | doc "Group is the group of the object being referenced."
      | optional,
    name
      | String
      | doc "Name is the name of the object being referenced.",
    namespace
      | String
      | doc "Namespace is the namespace of the object being referenced."
      | optional,
    resource
      | String
      | doc "Resource is the resource of the object being referenced."
  },

  ServiceCIDR = {
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
      | ServiceCIDRSpec
      | doc m%"
spec is the desired state of the ServiceCIDR. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
      | optional,
    status
      | ServiceCIDRStatus
      | doc m%"
status represents the current state of the ServiceCIDR. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
      | optional
  },

  ServiceCIDRList = {
    apiVersion
      | String
      | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
      | optional,
    items
      | Array ServiceCIDR
      | doc "items is the list of ServiceCIDRs.",
    kind
      | String
      | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
      | optional,
    metadata
      | listMeta
      | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
      | optional
  },

  ServiceCIDRSpec = {
    cidrs
      | Array String
      | doc m%"
CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64") from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family. This field is immutable.
"%
      | optional
  },

  ServiceCIDRStatus = {
    conditions
      | Array condition
      | doc m%"
conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR. Current service state
"%
      | optional
  }
}
