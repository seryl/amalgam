let aggregationRule = import "./AggregationRule.ncl" in
let objectMeta = import "./ObjectMeta.ncl" in
let policyRule = import "./PolicyRule.ncl" in

# Module: k8s.io.v1

# ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
{
  aggregationRule
     | aggregationRule
     | doc m%"
AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
"%
     | optional,
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | objectMeta
     | doc "Standard object's metadata."
     | optional,
  rules
     | Array policyRule
     | doc "Rules holds all the PolicyRules for this ClusterRole"
     | optional
}