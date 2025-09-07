# Module: k8s.io.v1

# ResourceFieldSelector represents container resources (cpu, memory) and their output format
{
  containerName
     | String
     | doc "Container name: required for volumes, optional for env vars"
     | optional,
  divisor
     | String
     | doc "Specifies the output format of the exposed resources, defaults to \"1\""
     | optional,
  resource
     | String
     | doc "Required: resource to select"
}