let nodeAddress = import "./NodeAddress.ncl" in
let nodeCondition = import "./NodeCondition.ncl" in
let nodeConfigStatus = import "./NodeConfigStatus.ncl" in
let nodeDaemonEndpoints = import "./NodeDaemonEndpoints.ncl" in
let nodeFeatures = import "./NodeFeatures.ncl" in
let containerImage = import "./ContainerImage.ncl" in
let nodeSystemInfo = import "./NodeSystemInfo.ncl" in
let nodeRuntimeHandler = import "./NodeRuntimeHandler.ncl" in
let attachedVolume = import "./AttachedVolume.ncl" in

# Module: k8s.io.v1

# NodeStatus is information about the current status of a node.
{
  addresses
     | Array nodeAddress
     | doc m%"
List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/reference/node/node-status/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
"%
     | optional,
  allocatable
     | { .. }
     | doc m%"
Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
"%
     | optional,
  capacity
     | { .. }
     | doc m%"
Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity
"%
     | optional,
  conditions
     | Array nodeCondition
     | doc m%"
Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/reference/node/node-status/#condition
"%
     | optional,
  config
     | nodeConfigStatus
     | doc m%"
Status of the config assigned to the node via the dynamic Kubelet config feature.
"%
     | optional,
  daemonEndpoints
     | nodeDaemonEndpoints
     | doc "Endpoints of daemons running on the Node."
     | optional,
  features
     | nodeFeatures
     | doc "Features describes the set of features implemented by the CRI implementation."
     | optional,
  images
     | Array containerImage
     | doc "List of container images on this node"
     | optional,
  nodeInfo
     | nodeSystemInfo
     | doc m%"
Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/reference/node/node-status/#info
"%
     | optional,
  phase
     | String
     | doc m%"
NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
"%
     | optional,
  runtimeHandlers
     | Array nodeRuntimeHandler
     | doc "The available runtime handlers."
     | optional,
  volumesAttached
     | Array attachedVolume
     | doc "List of volumes that are attached to the node."
     | optional,
  volumesInUse
     | Array String
     | doc "List of attachable volumes in use (mounted) by the node."
     | optional
}