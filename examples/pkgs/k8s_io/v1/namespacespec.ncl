# Module: k8s.io.v1

# NamespaceSpec describes the attributes on a Namespace.
{
  finalizers
     | Array String
     | doc m%"
Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%
     | optional
}