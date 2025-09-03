let deviceclaim = import "./deviceclaim.ncl" in

# Module: k8s.io.v1alpha3

# ResourceClaimSpec defines what is being requested in a ResourceClaim and how to configure it.
{
  devices | optional | deviceclaim | doc "Devices defines how to request devices."
}