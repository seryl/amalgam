# Module: k8s.io.v1

# CrossVersionObjectReference contains enough information to let you identify the referred resource.
{
  apiVersion | optional | String | doc "apiVersion is the API version of the referent",
  kind | String | doc m%"
kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  name | String | doc m%"
name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
}