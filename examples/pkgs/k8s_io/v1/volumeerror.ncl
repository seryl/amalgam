# Module: k8s.io.v1

# VolumeError captures an error encountered during a volume operation.
{
  errorCode | optional | Number | doc m%"
errorCode is a numeric gRPC code representing the error encountered during Attach or Detach operations.
This is an optional, alpha field that requires the MutableCSINodeAllocatableCount feature gate being enabled to be set.
"%,
  message | optional | String | doc m%"
message represents the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.
"%,
  time | optional | String | doc "time represents the time the error was encountered."
}