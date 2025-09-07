let cELDeviceSelector = import "./CELDeviceSelector.ncl" in

# Module: k8s.io.v1alpha3

# DeviceSelector must have exactly one field set.
{
  cel
     | cELDeviceSelector
     | doc "CEL contains a CEL expression for selecting a device."
     | optional
}