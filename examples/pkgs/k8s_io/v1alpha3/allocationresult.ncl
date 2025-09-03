let deviceallocationresult = import "./deviceallocationresult.ncl" in
let nodeselector = import "../v1/nodeselector.ncl" in

# Module: k8s.io.v1alpha3

# AllocationResult contains attributes of an allocated resource.
{
  devices | optional | deviceallocationresult | doc "Devices is the result of allocating devices.",
  nodeSelector | optional | nodeselector | doc m%"
NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
"%
}