# Module: k8s.io.v1

# NodeAddress contains information for the node's address.
{
  address | optional | String | doc "The node address.",
  type | optional | String | doc "Node address type, one of Hostname, ExternalIP or InternalIP."
}