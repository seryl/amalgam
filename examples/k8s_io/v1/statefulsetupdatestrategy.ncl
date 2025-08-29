# Module: k8s.io.v1.statefulsetupdatestrategy

let rollingupdatestatefulsetstrategy = import "./rollingupdatestatefulsetstrategy.ncl" in

{
  # StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
  StatefulSetUpdateStrategy = {
    rollingUpdate | optional | rollingupdatestatefulsetstrategy.RollingUpdateStatefulSetStrategy | doc m%"
RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
"%,
    type | optional | String | doc m%"
Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
"%,
  },
}
