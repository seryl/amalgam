# Module: k8s.io.v1.volumeerror

{
  # VolumeError captures an error encountered during a volume operation.
  VolumeError = {
    message | optional | String | doc m%"
message represents the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.
"%,
    time | optional | String | doc "time represents the time the error was encountered.",
  },
}
