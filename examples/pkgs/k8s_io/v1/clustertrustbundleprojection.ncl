let labelSelector = import "./LabelSelector.ncl" in

# Module: k8s.io.v1

# ClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects and project their contents into the pod filesystem.
{
  labelSelector
     | labelSelector
     | doc m%"
Select all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as "match nothing".  If set but empty, interpreted as "match everything".
"%
     | optional,
  name
     | String
     | doc m%"
Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.
"%
     | optional,
  "optional"
     | Bool
     | doc m%"
If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.
"%
     | optional,
  path
     | String
     | doc "Relative path from the volume root to write the bundle.",
  signerName
     | String
     | doc m%"
Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.
"%
     | optional
}