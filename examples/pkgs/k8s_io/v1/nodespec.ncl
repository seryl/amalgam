let nodeConfigSource = import "./NodeConfigSource.ncl" in
let taint = import "./Taint.ncl" in

# Module: k8s.io.v1

# NodeSpec describes the attributes that a node is created with.
{
  configSource
     | nodeConfigSource
     | doc m%"
Deprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.
"%
     | optional,
  externalID
     | String
     | doc m%"
Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
"%
     | optional,
  podCIDR
     | String
     | doc "PodCIDR represents the pod IP range assigned to the node."
     | optional,
  podCIDRs
     | Array String
     | doc m%"
podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
"%
     | optional,
  providerID
     | String
     | doc m%"
ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
"%
     | optional,
  taints
     | Array taint
     | doc "If specified, the node's taints."
     | optional,
  unschedulable
     | Bool
     | doc m%"
Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
"%
     | optional
}