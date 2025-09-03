# DeviceAllocationResult is the result of allocating devices.
{
  config | optional | Array DeviceAllocationConfiguration | doc m%"
This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.

This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.
"%,
  results | optional | Array DeviceRequestAllocationResult | doc "Results lists all allocated devices.",
}
