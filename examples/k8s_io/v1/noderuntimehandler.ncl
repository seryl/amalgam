# Module: k8s.io.v1.noderuntimehandler

let noderuntimehandlerfeatures = import "./noderuntimehandlerfeatures.ncl" in

{
  # NodeRuntimeHandler is a set of runtime handler information.
  NodeRuntimeHandler = {
    features | optional | noderuntimehandlerfeatures.NodeRuntimeHandlerFeatures | doc "Supported features.",
    name | optional | String | doc "Runtime handler name. Empty for the default runtime handler.",
  },
}
