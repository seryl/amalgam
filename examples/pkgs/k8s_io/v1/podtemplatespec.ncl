# Module: k8s.io.v1.podtemplatespec

let objectmeta = import "./objectmeta.ncl" in
let podspec = import "./podspec.ncl" in

{
  # PodTemplateSpec describes the data a pod should have when created from a template
  PodTemplateSpec = {
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    spec | optional | podspec.PodSpec | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
  },
}
