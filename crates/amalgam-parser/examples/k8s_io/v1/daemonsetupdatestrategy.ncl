# DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
{
  rollingUpdate | optional | RollingUpdateDaemonSet | doc "Rolling update config params. Present only if type = \"RollingUpdate\".",
  type | optional | String | doc m%"
Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
"%,
}
