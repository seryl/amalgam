let condition = import "../v1/Condition.ncl" in
let typeChecking = import "./TypeChecking.ncl" in

# Module: k8s.io.v1beta1

# ValidatingAdmissionPolicyStatus represents the status of an admission validation policy.
{
  conditions | optional | Array condition | doc m%"
The conditions represent the latest available observations of a policy's current state.
"%,
  observedGeneration | optional | Number | doc "The generation observed by the controller.",
  typeChecking | optional | typeChecking | doc m%"
The results of type checking for each expression. Presence of this field indicates the completion of the type checking.
"%
}