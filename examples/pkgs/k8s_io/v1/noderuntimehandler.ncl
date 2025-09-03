let noderuntimehandlerfeatures = import "./noderuntimehandlerfeatures.ncl" in

# Module: k8s.io.v1

# NodeRuntimeHandler is a set of runtime handler information.
{
  features | optional | noderuntimehandlerfeatures | doc "Supported features.",
  name | optional | String | doc "Runtime handler name. Empty for the default runtime handler."
}