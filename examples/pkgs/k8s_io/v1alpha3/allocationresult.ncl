let deviceAllocationResult = import "./DeviceAllocationResult.ncl" in
let nodeSelector = import "../v1/NodeSelector.ncl" in

# Module: k8s.io.v1alpha3

# AllocationResult contains attributes of an allocated resource.
{
  devices | optional | deviceAllocationResult | doc "Devices is the result of allocating devices.",
  nodeSelector | optional | nodeSelector | doc m%"
NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
"%
}