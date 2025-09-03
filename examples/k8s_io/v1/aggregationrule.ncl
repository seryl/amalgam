# AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
{
  clusterRoleSelectors | optional | Array LabelSelector | doc m%"
ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
"%,
}
