# Module: k8s.io.v1.nodeaddress

{
  # NodeAddress contains information for the node's address.
  NodeAddress = {
    address | String | doc "The node address.",
    type | String | doc "Node address type, one of Hostname, ExternalIP or InternalIP.",
  },
}
