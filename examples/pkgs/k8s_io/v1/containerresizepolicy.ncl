# Module: k8s.io.v1.containerresizepolicy

{
  # ContainerResizePolicy represents resource resize policy for the container.
  ContainerResizePolicy = {
    resourceName | optional | String | doc m%"
Name of the resource to which this resource resize policy applies. Supported values: cpu, memory.
"%,
    restartPolicy | optional | String | doc m%"
Restart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.
"%,
  },
}
