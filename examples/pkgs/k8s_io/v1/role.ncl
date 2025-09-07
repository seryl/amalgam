let objectMeta = import "./ObjectMeta.ncl" in
let policyRule = import "./PolicyRule.ncl" in

# Module: k8s.io.v1

# Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectMeta | doc "Standard object's metadata.",
  rules | optional | Array policyRule | doc "Rules holds all the PolicyRules for this Role"
}