let nodeAffinity = import "./NodeAffinity.ncl" in
let podAffinity = import "./PodAffinity.ncl" in
let podAntiAffinity = import "./PodAntiAffinity.ncl" in

# Module: k8s.io.v1

# Affinity is a group of affinity scheduling rules.
{
  nodeAffinity | optional | nodeAffinity | doc "Describes node affinity scheduling rules for the pod.",
  podAffinity | optional | podAffinity | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%,
  podAntiAffinity | optional | podAntiAffinity | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%
}