# Module: k8s.io.v1

# ResourceClaim references one entry in PodSpec.ResourceClaims.
{
  name | optional | String | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
"%,
  request | optional | String | doc m%"
Request is the name chosen for a request in the referenced claim. If empty, everything from the claim is made available, otherwise only the result of this request.
"%
}