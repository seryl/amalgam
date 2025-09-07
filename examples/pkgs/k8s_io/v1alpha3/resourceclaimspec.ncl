let deviceClaim = import "./DeviceClaim.ncl" in

# Module: k8s.io.v1alpha3

# ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.
{
  devices
     | deviceClaim
     | doc "Devices defines how to request devices."
     | optional
}