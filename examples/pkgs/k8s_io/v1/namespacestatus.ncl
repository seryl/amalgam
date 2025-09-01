# Module: k8s.io.v1.namespacestatus

let namespacecondition = import "./namespacecondition.ncl" in

{
  # NamespaceStatus is information about the current status of a Namespace.
  NamespaceStatus = {
    conditions | optional | Array namespacecondition.NamespaceCondition | doc "Represents the latest available observations of a namespace's current state.",
    phase | optional | String | doc m%"
Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%,
  },
}
