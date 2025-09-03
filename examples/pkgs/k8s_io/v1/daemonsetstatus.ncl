let daemonsetcondition = import "./daemonsetcondition.ncl" in

# Module: k8s.io.v1

# DaemonSetStatus represents the current status of a daemon set.
{
  collisionCount | optional | Number | doc m%"
Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
"%,
  conditions | optional | Array daemonsetcondition | doc "Represents the latest available observations of a DaemonSet's current state.",
  currentNumberScheduled | optional | Number | doc m%"
The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
  desiredNumberScheduled | optional | Number | doc m%"
The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
  numberAvailable | optional | Number | doc m%"
The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
"%,
  numberMisscheduled | optional | Number | doc m%"
The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
  numberReady | optional | Number | doc m%"
numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running with a Ready Condition.
"%,
  numberUnavailable | optional | Number | doc m%"
The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
"%,
  observedGeneration | optional | Number | doc "The most recent generation observed by the daemon set controller.",
  updatedNumberScheduled | optional | Number | doc "The total number of nodes that are running updated daemon pod"
}