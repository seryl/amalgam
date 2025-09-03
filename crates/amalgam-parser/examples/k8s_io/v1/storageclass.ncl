# StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
# 
# StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
{
  allowVolumeExpansion | optional | Bool | doc "allowVolumeExpansion shows whether the storage class allow volume expand.",
  allowedTopologies | optional | Array TopologySelectorTerm | doc m%"
allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
"%,
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  mountOptions | optional | Array String | doc m%"
mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. ["ro", "soft"]. Not validated - mount of the PVs will simply fail if one is invalid.
"%,
  parameters | optional | { .. } | doc m%"
parameters holds the parameters for the provisioner that should create volumes of this storage class.
"%,
  provisioner | optional | String | doc "provisioner indicates the type of the provisioner.",
  reclaimPolicy | optional | String | doc m%"
reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes of this storage class. Defaults to Delete.
"%,
  volumeBindingMode | optional | String | doc m%"
volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
"%,
}
