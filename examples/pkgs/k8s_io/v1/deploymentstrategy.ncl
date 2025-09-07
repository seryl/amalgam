let rollingUpdateDeployment = import "./RollingUpdateDeployment.ncl" in

# Module: k8s.io.v1

# DeploymentStrategy describes how to replace existing pods with new ones.
{
  rollingUpdate
     | rollingUpdateDeployment
     | doc m%"
Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
"%
     | optional,
  type
     | String
     | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%
     | optional
}