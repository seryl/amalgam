# Module: k8s.io.v1.namespacespec

{
  # NamespaceSpec describes the attributes on a Namespace.
  NamespaceSpec = {
    finalizers | optional | Array String | doc m%"
Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%,
  },
}
