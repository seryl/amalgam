let clusterTrustBundleProjection = import "./ClusterTrustBundleProjection.ncl" in
let configMapProjection = import "./ConfigMapProjection.ncl" in
let downwardAPIProjection = import "./DownwardAPIProjection.ncl" in
let secretProjection = import "./SecretProjection.ncl" in
let serviceAccountTokenProjection = import "./ServiceAccountTokenProjection.ncl" in

# Module: k8s.io.v1

# Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.
{
  clusterTrustBundle | optional | clusterTrustBundleProjection | doc m%"
ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.
Alpha, gated by the ClusterTrustBundleProjection feature gate.
ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.
Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
"%,
  configMap | optional | configMapProjection | doc "configMap information about the configMap data to project",
  downwardAPI | optional | downwardAPIProjection | doc "downwardAPI information about the downwardAPI data to project",
  secret | optional | secretProjection | doc "secret information about the secret data to project",
  serviceAccountToken | optional | serviceAccountTokenProjection | doc "serviceAccountToken is information about the serviceAccountToken data to project"
}