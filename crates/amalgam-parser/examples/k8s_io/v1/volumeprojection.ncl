# Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.
{
  clusterTrustBundle | optional | ClusterTrustBundleProjection | doc m%"
ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.

Alpha, gated by the ClusterTrustBundleProjection feature gate.

ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.

Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
"%,
  configMap | optional | ConfigMapProjection | doc "configMap information about the configMap data to project",
  downwardAPI | optional | DownwardAPIProjection | doc "downwardAPI information about the downwardAPI data to project",
  secret | optional | SecretProjection | doc "secret information about the secret data to project",
  serviceAccountToken | optional | ServiceAccountTokenProjection | doc "serviceAccountToken is information about the serviceAccountToken data to project",
}
