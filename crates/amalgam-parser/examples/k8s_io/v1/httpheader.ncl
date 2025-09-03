# HTTPHeader describes a custom header to be used in HTTP probes
{
  name | optional | String | doc m%"
The header field name. This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
  value | optional | String | doc "The header field value",
}
