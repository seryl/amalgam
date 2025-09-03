# Module: k8s.io.v1

# IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
{
  cidr | optional | String | doc m%"
cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
"%,
  except | optional | Array String | doc m%"
except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
"%
}
