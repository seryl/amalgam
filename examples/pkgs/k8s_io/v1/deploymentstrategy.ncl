let rollingupdatedeployment = import "./rollingupdatedeployment.ncl" in

# Module: k8s.io.v1

# DeploymentStrategy describes how to replace existing pods with new ones.
{
  rollingUpdate | optional | rollingupdatedeployment | doc m%"
Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
"%,
  type | optional | String | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%
}