let labelSelector = import "./LabelSelector.ncl" in

# Module: k8s.io.v1

# AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
{
  clusterRoleSelectors | optional | Array labelSelector | doc m%"
ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
"%
}