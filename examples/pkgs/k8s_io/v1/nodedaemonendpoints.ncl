let daemonEndpoint = import "./DaemonEndpoint.ncl" in

# Module: k8s.io.v1

# NodeDaemonEndpoints lists ports opened by daemons running on the Node.
{
  kubeletEndpoint | optional | daemonEndpoint | doc "Endpoint on which Kubelet is listening."
}