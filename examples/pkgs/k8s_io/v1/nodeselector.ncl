let nodeselectorterm = import "./nodeselectorterm.ncl" in

# Module: k8s.io.v1

# A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
{
  nodeSelectorTerms | optional | Array nodeselectorterm | doc "Required. A list of node selector terms. The terms are ORed."
}