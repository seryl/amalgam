# Module: k8s.io.v1.httpheader

{
  # HTTPHeader describes a custom header to be used in HTTP probes
  HTTPHeader = {
    name | String | doc m%"
The header field name. This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
    value | String | doc "The header field value",
  },
}
