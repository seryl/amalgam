let objectmeta = import "./objectmeta.ncl" in
let jobspec = import "./jobspec.ncl" in

# Module: k8s.io.v1

# JobTemplateSpec describes the data a Job should have when created from a template
{
  metadata | optional | objectmeta | doc m%"
Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | jobspec | doc m%"
Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
}