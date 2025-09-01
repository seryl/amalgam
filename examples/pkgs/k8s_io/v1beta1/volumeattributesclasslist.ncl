# Module: k8s.io.v1beta1.volumeattributesclasslist

let ListMeta = import "../v1/listmeta.ncl" in
let volumeattributesclass = import "./volumeattributesclass.ncl" in

{
  # VolumeAttributesClassList is a collection of VolumeAttributesClass objects.
  VolumeAttributesClassList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array volumeattributesclass.VolumeAttributesClass | doc "items is the list of VolumeAttributesClass objects.",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | ListMeta | doc m%"
Standard list metadata More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  },
}
