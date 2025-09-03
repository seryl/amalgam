# Module: k8s.io.v1

# NodeDaemonEndpoints lists ports opened by daemons running on the Node.
{
  kubeletEndpoint | optional | daemonendpoint.DaemonEndpoint | doc "Endpoint on which Kubelet is listening."
}
