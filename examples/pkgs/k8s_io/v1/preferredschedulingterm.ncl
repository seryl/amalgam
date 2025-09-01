# Module: k8s.io.v1.preferredschedulingterm

let nodeselectorterm = import "./nodeselectorterm.ncl" in

{
  # An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
  PreferredSchedulingTerm = {
    preference | optional | nodeselectorterm.NodeSelectorTerm | doc "A node selector term, associated with the corresponding weight.",
    weight | optional | Number | doc m%"
Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
"%,
  },
}
