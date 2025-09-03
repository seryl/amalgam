# PodTemplateSpec describes the data a pod should have when created from a template
{
  metadata | optional | ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | PodSpec | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%,
}
