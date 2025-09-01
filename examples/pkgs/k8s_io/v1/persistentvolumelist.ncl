# Module: k8s.io.v1.persistentvolumelist

let listmeta = import "./listmeta.ncl" in
let persistentvolume = import "./persistentvolume.ncl" in

{
  # PersistentVolumeList is a list of PersistentVolume items.
  PersistentVolumeList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array persistentvolume.PersistentVolume | doc m%"
items is a list of persistent volumes. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  },
}
