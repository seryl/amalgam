# Module: k8s.io.v1.noderuntimehandlerfeatures

{
  # NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.
  NodeRuntimeHandlerFeatures = {
    recursiveReadOnlyMounts | optional | Bool | doc m%"
RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
"%,
    userNamespaces | optional | Bool | doc m%"
UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
"%,
  },
}
