let expressionwarning = import "./expressionwarning.ncl" in

# Module: k8s.io.v1beta1

# TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy
{
  expressionWarnings | optional | Array expressionwarning | doc "The type checking warnings for each expression."
}