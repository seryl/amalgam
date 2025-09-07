let objectMeta = import "./ObjectMeta.ncl" in
let jobSpec = import "./JobSpec.ncl" in

# Module: k8s.io.v1

# JobTemplateSpec describes the data a Job should have when created from a template
{
  metadata | optional | objectMeta | doc m%"
Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | jobSpec | doc m%"
Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
}