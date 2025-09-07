# Module: k8s.io.v1

# Adds and removes POSIX capabilities from running containers.
{
  add
     | Array String
     | doc "Added capabilities"
     | optional,
  drop
     | Array String
     | doc "Removed capabilities"
     | optional
}