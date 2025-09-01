# Module: k8s.io.v1.capabilities

{
  # Adds and removes POSIX capabilities from running containers.
  Capabilities = {
    add | optional | Array String | doc "Added capabilities",
    drop | optional | Array String | doc "Removed capabilities",
  },
}
