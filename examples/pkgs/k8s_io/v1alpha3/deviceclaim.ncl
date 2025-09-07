let deviceClaimConfiguration = import "./DeviceClaimConfiguration.ncl" in
let deviceConstraint = import "./DeviceConstraint.ncl" in
let deviceRequest = import "./DeviceRequest.ncl" in

# Module: k8s.io.v1alpha3

# DeviceClaim defines how to request devices with a ResourceClaim.
{
  config
     | Array deviceClaimConfiguration
     | doc m%"
This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.
"%
     | optional,
  constraints
     | Array deviceConstraint
     | doc m%"
These constraints must be satisfied by the set of devices that get allocated for the claim.
"%
     | optional,
  requests
     | Array deviceRequest
     | doc m%"
Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.
"%
     | optional
}