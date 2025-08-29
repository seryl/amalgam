# Module: k8s.io.v1.daemonsetupdatestrategy

let rollingupdatedaemonset = import "./rollingupdatedaemonset.ncl" in

{
  # DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
  DaemonSetUpdateStrategy = {
    rollingUpdate | optional | rollingupdatedaemonset.RollingUpdateDaemonSet | doc "Rolling update config params. Present only if type = \"RollingUpdate\".",
    type | optional | String | doc m%"
Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
"%,
  },
}
