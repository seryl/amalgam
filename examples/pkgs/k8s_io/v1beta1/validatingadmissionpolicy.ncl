let objectMeta = import "../v1/ObjectMeta.ncl" in
let validatingAdmissionPolicySpec = import "./ValidatingAdmissionPolicySpec.ncl" in
let validatingAdmissionPolicyStatus = import "./ValidatingAdmissionPolicyStatus.ncl" in

# Module: k8s.io.v1beta1

# ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
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
     | validatingAdmissionPolicySpec
     | doc "Specification of the desired behavior of the ValidatingAdmissionPolicy."
     | optional,
  status
     | validatingAdmissionPolicyStatus
     | doc m%"
The status of the ValidatingAdmissionPolicy, including warnings that are useful to determine if the policy behaves in the expected way. Populated by the system. Read-only.
"%
     | optional
}