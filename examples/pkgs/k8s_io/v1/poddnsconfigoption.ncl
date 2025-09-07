# Module: k8s.io.v1

# PodDNSConfigOption defines DNS resolver options of a pod.
{
  name
     | String
     | doc "Name is this DNS resolver option's name. Required."
     | optional,
  value
     | String
     | doc "Value is this DNS resolver option's value."
     | optional
}