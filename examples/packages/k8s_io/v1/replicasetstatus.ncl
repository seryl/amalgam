# Module: k8s.io.v1.replicasetstatus

let replicasetcondition = import "./replicasetcondition.ncl" in

{
  # ReplicaSetStatus represents the current status of a ReplicaSet.
  ReplicaSetStatus = {
    availableReplicas | optional | Number | doc m%"
The number of available replicas (ready for at least minReadySeconds) for this replica set.
"%,
    conditions | optional | Array replicasetcondition.ReplicaSetCondition | doc "Represents the latest available observations of a replica set's current state.",
    fullyLabeledReplicas | optional | Number | doc m%"
The number of pods that have labels matching the labels of the pod template of the replicaset.
"%,
    observedGeneration | optional | Number | doc m%"
ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
"%,
    readyReplicas | optional | Number | doc m%"
readyReplicas is the number of pods targeted by this ReplicaSet with a Ready Condition.
"%,
    replicas | Number | doc m%"
Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
"%,
  },
}
