let nodeSelector = import "./NodeSelector.ncl" in

# Module: k8s.io.v1

# VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
{
  required
     | nodeSelector
     | doc "required specifies hard node constraints that must be met."
     | optional
}