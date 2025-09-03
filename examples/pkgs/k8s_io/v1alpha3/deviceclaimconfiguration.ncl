let opaquedeviceconfiguration = import "./opaquedeviceconfiguration.ncl" in

# Module: k8s.io.v1alpha3

# DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.
{
  opaque | optional | opaquedeviceconfiguration | doc "Opaque provides driver-specific configuration parameters.",
  requests | optional | Array String | doc m%"
Requests lists the names of requests where the configuration applies. If empty, it applies to all requests.
References to subrequests must include the name of the main request and may include the subrequest using the format <main request>[/<subrequest>]. If just the main request is given, the configuration applies to all subrequests.
"%
}