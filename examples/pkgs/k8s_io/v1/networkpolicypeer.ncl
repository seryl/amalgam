# Module: k8s.io.v1.networkpolicypeer

let labelselector = import "./labelselector.ncl" in
let ipblock = import "./ipblock.ncl" in

{
  # NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
  NetworkPolicyPeer = {
    ipBlock | optional | ipblock.IPBlock | doc m%"
ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
"%,
    namespaceSelector | optional | labelselector.LabelSelector | doc m%"
namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.

If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
"%,
    podSelector | optional | labelselector.LabelSelector | doc m%"
podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods.

If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
"%,
  },
}
