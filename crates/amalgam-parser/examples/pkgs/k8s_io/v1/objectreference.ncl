# Module: k8s.io.v1

# ObjectReference contains enough information to let you inspect or modify the referred object.
{
  apiVersion | optional | String | doc "API version of the referent.",
  fieldPath | optional | String | doc m%"
If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.
"%,
  kind | optional | String | doc m%"
Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  name | optional | String | doc m%"
Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%,
  namespace | optional | String | doc m%"
Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
"%,
  resourceVersion | optional | String | doc m%"
Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
"%,
  uid | optional | String | doc m%"
UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
"%
}
