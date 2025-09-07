let rollingUpdateDaemonSet = import "./RollingUpdateDaemonSet.ncl" in

# Module: k8s.io.v1

# DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
{
  rollingUpdate
     | rollingUpdateDaemonSet
     | doc "Rolling update config params. Present only if type = \"RollingUpdate\"."
     | optional,
  type
     | String
     | doc m%"
Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
"%
     | optional
}