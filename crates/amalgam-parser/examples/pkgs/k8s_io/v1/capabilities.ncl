# Module: k8s.io.v1

# Adds and removes POSIX capabilities from running containers.
{
  add | optional | Array String | doc "Added capabilities",
  drop | optional | Array String | doc "Removed capabilities"
}
