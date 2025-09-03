# Module: k8s.io.v1

# A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
{
  matchExpressions | optional | Array nodeselectorrequirement.NodeSelectorRequirement | doc "A list of node selector requirements by node's labels.",
  matchFields | optional | Array nodeselectorrequirement.NodeSelectorRequirement | doc "A list of node selector requirements by node's fields."
}
