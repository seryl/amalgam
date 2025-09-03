# DeploymentStrategy describes how to replace existing pods with new ones.
{
  rollingUpdate | optional | RollingUpdateDeployment | doc m%"
Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
"%,
  type | optional | String | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%,
}
