# Module: k8s.io.v1.podschedulinggate

{
  # PodSchedulingGate is associated to a Pod to guard its scheduling.
  PodSchedulingGate = {
    name | String | doc "Name of the scheduling gate. Each scheduling gate must have a unique name field.",
  },
}
