# The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
{
  podAffinityTerm | optional | PodAffinityTerm | doc "Required. A pod affinity term, associated with the corresponding weight.",
  weight | optional | Number | doc m%"
weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
"%,
}
