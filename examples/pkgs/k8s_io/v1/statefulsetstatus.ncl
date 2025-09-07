let statefulSetCondition = import "./StatefulSetCondition.ncl" in

# Module: k8s.io.v1

# StatefulSetStatus represents the current state of a StatefulSet.
{
  availableReplicas | optional | Number | doc m%"
Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
"%,
  collisionCount | optional | Number | doc m%"
collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
"%,
  conditions | optional | Array statefulSetCondition | doc "Represents the latest available observations of a statefulset's current state.",
  currentReplicas | optional | Number | doc m%"
currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
"%,
  currentRevision | optional | String | doc m%"
currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).
"%,
  observedGeneration | optional | Number | doc m%"
observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
"%,
  readyReplicas | optional | Number | doc m%"
readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
"%,
  replicas | Number | doc "replicas is the number of Pods created by the StatefulSet controller.",
  updateRevision | optional | String | doc m%"
updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
"%,
  updatedReplicas | optional | Number | doc m%"
updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
"%
}