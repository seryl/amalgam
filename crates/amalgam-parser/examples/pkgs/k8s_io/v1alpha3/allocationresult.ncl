# Module: k8s.io.v1alpha3

# AllocationResult contains attributes of an allocated resource.
{
  devices | optional | deviceallocationresult.DeviceAllocationResult | doc "Devices is the result of allocating devices.",
  nodeSelector | optional | nodeselector.NodeSelector | doc m%"
NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
"%
}
