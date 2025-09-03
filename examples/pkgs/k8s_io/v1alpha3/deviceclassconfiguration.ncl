let opaquedeviceconfiguration = import "./opaquedeviceconfiguration.ncl" in

# Module: k8s.io.v1alpha3

# DeviceClassConfiguration is used in DeviceClass.
{
  opaque | optional | opaquedeviceconfiguration | doc "Opaque provides driver-specific configuration parameters."
}