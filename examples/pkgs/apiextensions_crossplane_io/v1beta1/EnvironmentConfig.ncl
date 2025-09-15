let v1Module = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in

# Module: apiextensions.crossplane.io.v1beta1

{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  data
     | {
  }
     | doc m%"
The data of this EnvironmentConfig.
This may contain any kind of structure that can be serialized into JSON.
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | v1Module.ObjectMeta
     | optional
}