let objectmeta = import "../v1/objectmeta.ncl" in
let validatingadmissionpolicyspec = import "./validatingadmissionpolicyspec.ncl" in
let validatingadmissionpolicystatus = import "./validatingadmissionpolicystatus.ncl" in

# Module: k8s.io.v1beta1

# ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectmeta | doc m%"
Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
"%,
  spec | optional | validatingadmissionpolicyspec | doc "Specification of the desired behavior of the ValidatingAdmissionPolicy.",
  status | optional | validatingadmissionpolicystatus | doc m%"
The status of the ValidatingAdmissionPolicy, including warnings that are useful to determine if the policy behaves in the expected way. Populated by the system. Read-only.
"%
}