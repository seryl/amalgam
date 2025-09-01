# Module: k8s.io.v1.volumenodeaffinity

let nodeselector = import "./nodeselector.ncl" in

{
  # VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
  VolumeNodeAffinity = {
    required | optional | nodeselector.NodeSelector | doc "required specifies hard node constraints that must be met.",
  },
}
