let nodeaffinity = import "./nodeaffinity.ncl" in
let podaffinity = import "./podaffinity.ncl" in
let podantiaffinity = import "./podantiaffinity.ncl" in

# Module: k8s.io.v1

# Affinity is a group of affinity scheduling rules.
{
  nodeAffinity | optional | nodeaffinity | doc "Describes node affinity scheduling rules for the pod.",
  podAffinity | optional | podaffinity | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%,
  podAntiAffinity | optional | podantiaffinity | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%
}