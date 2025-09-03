# Module: k8s.io.v1

# PodDNSConfigOption defines DNS resolver options of a pod.
{
  name | optional | String | doc "Name is this DNS resolver option's name. Required.",
  value | optional | String | doc "Value is this DNS resolver option's value."
}
