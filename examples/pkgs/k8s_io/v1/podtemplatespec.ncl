let objectMeta = import "./ObjectMeta.ncl" in
let podSpec = import "./PodSpec.ncl" in

# Module: k8s.io.v1

# PodTemplateSpec describes the data a pod should have when created from a template
{
  metadata | optional | objectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | podSpec | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
}