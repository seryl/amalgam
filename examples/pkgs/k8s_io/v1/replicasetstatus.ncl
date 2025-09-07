let replicaSetCondition = import "./ReplicaSetCondition.ncl" in

# Module: k8s.io.v1

# ReplicaSetStatus represents the current status of a ReplicaSet.
{
  availableReplicas | optional | Number | doc m%"
The number of available non-terminating pods (ready for at least minReadySeconds) for this replica set.
"%,
  conditions | optional | Array replicaSetCondition | doc "Represents the latest available observations of a replica set's current state.",
  fullyLabeledReplicas | optional | Number | doc m%"
The number of non-terminating pods that have labels matching the labels of the pod template of the replicaset.
"%,
  observedGeneration | optional | Number | doc m%"
ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
"%,
  readyReplicas | optional | Number | doc m%"
The number of non-terminating pods targeted by this ReplicaSet with a Ready Condition.
"%,
  replicas | Number | doc m%"
Replicas is the most recently observed number of non-terminating pods. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset
"%,
  terminatingReplicas | optional | Number | doc m%"
The number of terminating pods for this replica set. Terminating pods have a non-null .metadata.deletionTimestamp and have not yet reached the Failed or Succeeded .status.phase.
This is an alpha field. Enable DeploymentReplicaSetTerminatingReplicas to be able to use this field.
"%
}