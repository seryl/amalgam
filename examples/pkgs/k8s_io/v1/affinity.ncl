# Module: k8s.io.v1.affinity

let podaffinity = import "./podaffinity.ncl" in
let podantiaffinity = import "./podantiaffinity.ncl" in
let nodeaffinity = import "./nodeaffinity.ncl" in

{
  # Affinity is a group of affinity scheduling rules.
  Affinity = {
    nodeAffinity | optional | nodeaffinity.NodeAffinity | doc "Describes node affinity scheduling rules for the pod.",
    podAffinity | optional | podaffinity.PodAffinity | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%,
    podAntiAffinity | optional | podantiaffinity.PodAntiAffinity | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%,
  },
}
