let opaqueDeviceConfiguration = import "./OpaqueDeviceConfiguration.ncl" in

# Module: k8s.io.v1beta1

# DeviceClassConfiguration is used in DeviceClass.
{
  opaque | optional | opaqueDeviceConfiguration | doc "Opaque provides driver-specific configuration parameters."
}