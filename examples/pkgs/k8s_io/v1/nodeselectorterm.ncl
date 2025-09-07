let nodeSelectorRequirement = import "./NodeSelectorRequirement.ncl" in

# Module: k8s.io.v1

# A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
{
  matchExpressions
     | Array nodeSelectorRequirement
     | doc "A list of node selector requirements by node's labels."
     | optional,
  matchFields
     | Array nodeSelectorRequirement
     | doc "A list of node selector requirements by node's fields."
     | optional
}