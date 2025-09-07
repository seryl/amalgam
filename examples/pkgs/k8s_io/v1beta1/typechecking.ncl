let expressionWarning = import "./ExpressionWarning.ncl" in

# Module: k8s.io.v1beta1

# TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy
{
  expressionWarnings
     | Array expressionWarning
     | doc "The type checking warnings for each expression."
     | optional
}