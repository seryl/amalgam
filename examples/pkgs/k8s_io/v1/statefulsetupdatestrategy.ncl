let rollingUpdateStatefulSetStrategy = import "./RollingUpdateStatefulSetStrategy.ncl" in

# Module: k8s.io.v1

# StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
{
  rollingUpdate | optional | rollingUpdateStatefulSetStrategy | doc m%"
RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
"%,
  type | optional | String | doc m%"
Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
"%
}