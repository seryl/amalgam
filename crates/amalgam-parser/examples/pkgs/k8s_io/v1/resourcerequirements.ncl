# Module: k8s.io.v1

# ResourceRequirements describes the compute resource requirements.
{
  claims | optional | Array resourceclaim.ResourceClaim | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container.

This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.

This field is immutable. It can only be set for containers.
"%,
  limits | optional | { .. } | doc m%"
Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%,
  requests | optional | { .. } | doc m%"
Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
}
