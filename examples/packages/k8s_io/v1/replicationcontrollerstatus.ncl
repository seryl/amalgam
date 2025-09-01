# Module: k8s.io.v1.replicationcontrollerstatus

let replicationcontrollercondition = import "./replicationcontrollercondition.ncl" in

{
  # ReplicationControllerStatus represents the current status of a replication controller.
  ReplicationControllerStatus = {
    availableReplicas | optional | Number | doc m%"
The number of available replicas (ready for at least minReadySeconds) for this replication controller.
"%,
    conditions | optional | Array replicationcontrollercondition.ReplicationControllerCondition | doc m%"
Represents the latest available observations of a replication controller's current state.
"%,
    fullyLabeledReplicas | optional | Number | doc m%"
The number of pods that have labels matching the labels of the pod template of the replication controller.
"%,
    observedGeneration | optional | Number | doc m%"
ObservedGeneration reflects the generation of the most recently observed replication controller.
"%,
    readyReplicas | optional | Number | doc "The number of ready replicas for this replication controller.",
    replicas | Number | doc m%"
Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
"%,
  },
}
