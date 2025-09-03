# DeviceClaim defines how to request devices with a ResourceClaim.
{
  config | optional | Array DeviceClaimConfiguration | doc m%"
This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.
"%,
  constraints | optional | Array DeviceConstraint | doc m%"
These constraints must be satisfied by the set of devices that get allocated for the claim.
"%,
  requests | optional | Array DeviceRequest | doc m%"
Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.
"%,
}
