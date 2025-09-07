let namespaceCondition = import "./NamespaceCondition.ncl" in

# Module: k8s.io.v1

# NamespaceStatus is information about the current status of a Namespace.
{
  conditions
     | Array namespaceCondition
     | doc "Represents the latest available observations of a namespace's current state."
     | optional,
  phase
     | String
     | doc m%"
Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%
     | optional
}