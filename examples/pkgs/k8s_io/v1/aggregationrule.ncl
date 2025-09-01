# Module: k8s.io.v1.aggregationrule

let labelselector = import "./labelselector.ncl" in

{
  # AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
  AggregationRule = {
    clusterRoleSelectors | optional | Array labelselector.LabelSelector | doc m%"
ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
"%,
  },
}
