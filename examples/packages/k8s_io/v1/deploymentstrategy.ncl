# Module: k8s.io.v1.deploymentstrategy

let rollingupdatedeployment = import "./rollingupdatedeployment.ncl" in

{
  # DeploymentStrategy describes how to replace existing pods with new ones.
  DeploymentStrategy = {
    rollingUpdate | optional | rollingupdatedeployment.RollingUpdateDeployment | doc m%"
Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
"%,
    type | optional | String | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%,
  },
}
