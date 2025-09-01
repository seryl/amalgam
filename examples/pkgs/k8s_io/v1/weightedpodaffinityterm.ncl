# Module: k8s.io.v1.weightedpodaffinityterm

let podaffinityterm = import "./podaffinityterm.ncl" in

{
  # The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
  WeightedPodAffinityTerm = {
    podAffinityTerm | optional | podaffinityterm.PodAffinityTerm | doc "Required. A pod affinity term, associated with the corresponding weight.",
    weight | optional | Number | doc m%"
weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
"%,
  },
}
