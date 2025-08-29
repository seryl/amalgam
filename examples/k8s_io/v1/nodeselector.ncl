# Module: k8s.io.v1.nodeselector

let nodeselectorterm = import "./nodeselectorterm.ncl" in

{
  # A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
  NodeSelector = {
    nodeSelectorTerms | Array nodeselectorterm.NodeSelectorTerm | doc "Required. A list of node selector terms. The terms are ORed.",
  },
}
