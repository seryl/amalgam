# Module: k8s.io.v1.podip

{
  # PodIP represents a single IP address allocated to the pod.
  PodIP = {
    ip | optional | String | doc "IP is the IP address assigned to the pod",
  },
}
