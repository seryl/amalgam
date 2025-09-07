let objectMeta = import "./ObjectMeta.ncl" in
let podSpec = import "./PodSpec.ncl" in

# Module: k8s.io.v1

# PodTemplateSpec describes the data a pod should have when created from a template
{
  metadata
     | objectMeta
     | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
     | optional,
  spec
     | podSpec
     | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
     | optional
}