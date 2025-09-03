# A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
{
  matchExpressions | optional | Array NodeSelectorRequirement | doc "A list of node selector requirements by node's labels.",
  matchFields | optional | Array NodeSelectorRequirement | doc "A list of node selector requirements by node's fields.",
}
