# Affinity is a group of affinity scheduling rules.
{
  nodeAffinity | optional | NodeAffinity | doc "Describes node affinity scheduling rules for the pod.",
  podAffinity | optional | PodAffinity | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%,
  podAntiAffinity | optional | PodAntiAffinity | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%,
}
