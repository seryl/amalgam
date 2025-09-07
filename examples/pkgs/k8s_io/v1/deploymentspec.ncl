let labelSelector = import "./LabelSelector.ncl" in
let deploymentStrategy = import "./DeploymentStrategy.ncl" in
let podTemplateSpec = import "./PodTemplateSpec.ncl" in

# Module: k8s.io.v1

# DeploymentSpec is the specification of the desired behavior of the Deployment.
{
  minReadySeconds
     | Number
     | doc m%"
Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
"%
     | optional,
  paused
     | Bool
     | doc "Indicates that the deployment is paused."
     | optional,
  progressDeadlineSeconds
     | Number
     | doc m%"
The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s.
"%
     | optional,
  replicas
     | Number
     | doc m%"
Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
"%
     | optional,
  revisionHistoryLimit
     | Number
     | doc m%"
The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
"%
     | optional,
  selector
     | labelSelector
     | doc m%"
Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template's labels.
"%,
  strategy
     | deploymentStrategy
     | doc "The deployment strategy to use to replace existing pods with new ones."
     | optional,
  template
     | podTemplateSpec
     | doc m%"
Template describes the pods that will be created. The only allowed template.spec.restartPolicy value is "Always".
"%
}