# Module: k8s.io.v1

# NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.
{
  recursiveReadOnlyMounts
     | Bool
     | doc m%"
RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
"%
     | optional,
  userNamespaces
     | Bool
     | doc m%"
UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
"%
     | optional
}