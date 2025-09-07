let deploymentCondition = import "./DeploymentCondition.ncl" in

# Module: k8s.io.v1

# DeploymentStatus is the most recently observed status of the Deployment.
{
  availableReplicas
     | Number
     | doc m%"
Total number of available non-terminating pods (ready for at least minReadySeconds) targeted by this deployment.
"%
     | optional,
  collisionCount
     | Number
     | doc m%"
Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
"%
     | optional,
  conditions
     | Array deploymentCondition
     | doc "Represents the latest available observations of a deployment's current state."
     | optional,
  observedGeneration
     | Number
     | doc "The generation observed by the deployment controller."
     | optional,
  readyReplicas
     | Number
     | doc m%"
Total number of non-terminating pods targeted by this Deployment with a Ready Condition.
"%
     | optional,
  replicas
     | Number
     | doc m%"
Total number of non-terminating pods targeted by this deployment (their labels match the selector).
"%
     | optional,
  terminatingReplicas
     | Number
     | doc m%"
Total number of terminating pods targeted by this deployment. Terminating pods have a non-null .metadata.deletionTimestamp and have not yet reached the Failed or Succeeded .status.phase.
This is an alpha field. Enable DeploymentReplicaSetTerminatingReplicas to be able to use this field.
"%
     | optional,
  unavailableReplicas
     | Number
     | doc m%"
Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
"%
     | optional,
  updatedReplicas
     | Number
     | doc m%"
Total number of non-terminating pods targeted by this deployment that have the desired template spec.
"%
     | optional
}