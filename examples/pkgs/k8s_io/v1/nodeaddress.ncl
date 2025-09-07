# Module: k8s.io.v1

# NodeAddress contains information for the node's address.
{
  address
     | String
     | doc "The node address.",
  type
     | String
     | doc "Node address type, one of Hostname, ExternalIP or InternalIP."
}