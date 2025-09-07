let objectMeta = import "../v1/ObjectMeta.ncl" in
let mutatingAdmissionPolicySpec = import "./MutatingAdmissionPolicySpec.ncl" in

# Module: k8s.io.v1alpha1

# MutatingAdmissionPolicy describes the definition of an admission mutation policy that mutates the object coming into admission chain.
{
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
     | doc m%"
Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
"%
     | optional,
  spec
     | mutatingAdmissionPolicySpec
     | doc "Specification of the desired behavior of the MutatingAdmissionPolicy."
     | optional
}