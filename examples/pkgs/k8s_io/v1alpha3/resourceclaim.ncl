let objectMeta = import "../v1/ObjectMeta.ncl" in
let resourceClaimSpec = import "./ResourceClaimSpec.ncl" in
let resourceClaimStatus = import "./ResourceClaimStatus.ncl" in

# Module: k8s.io.v1alpha3

# ResourceClaim describes a request for access to resources in the cluster, for use by workloads. For example, if a workload needs an accelerator device with specific properties, this is how that request is expressed. The status stanza tracks whether this claim has been satisfied and what specific resources have been allocated.
# 
# This is an alpha type and requires enabling the DynamicResourceAllocation feature gate.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectMeta | doc "Standard object metadata",
  spec | resourceClaimSpec | doc m%"
Spec describes what is being requested and how to configure it. The spec is immutable.
"%,
  status | optional | resourceClaimStatus | doc "Status describes whether the claim is ready to use and what has been allocated."
}