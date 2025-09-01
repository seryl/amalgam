# Module: k8s.io.v1.poddnsconfigoption

{
  # PodDNSConfigOption defines DNS resolver options of a pod.
  PodDNSConfigOption = {
    name | optional | String | doc "Required.",
    value | optional | String,
  },
}
