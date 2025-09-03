# ResourceFieldSelector represents container resources (cpu, memory) and their output format
{
  containerName | optional | String | doc "Container name: required for volumes, optional for env vars",
  divisor | optional | String | doc "Specifies the output format of the exposed resources, defaults to \"1\"",
  resource | optional | String | doc "Required: resource to select",
}
