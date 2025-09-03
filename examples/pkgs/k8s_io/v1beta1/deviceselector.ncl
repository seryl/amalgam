let celdeviceselector = import "./celdeviceselector.ncl" in

# Module: k8s.io.v1beta1

# DeviceSelector must have exactly one field set.
{
  cel | optional | celdeviceselector | doc "CEL contains a CEL expression for selecting a device."
}