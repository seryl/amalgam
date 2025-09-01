# Module: k8s.io.v1.podtemplate

let podtemplatespec = import "./podtemplatespec.ncl" in
let objectmeta = import "./objectmeta.ncl" in

{
  # PodTemplate describes a template for creating copies of a predefined pod.
  PodTemplate = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    template | optional | podtemplatespec.PodTemplateSpec | doc m%"
Template defines the pods that will be created from this pod template. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
  },
}
