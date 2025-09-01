# Module: k8s.io.v1.nodedaemonendpoints

let daemonendpoint = import "./daemonendpoint.ncl" in

{
  # NodeDaemonEndpoints lists ports opened by daemons running on the Node.
  NodeDaemonEndpoints = {
    kubeletEndpoint | optional | daemonendpoint.DaemonEndpoint | doc "Endpoint on which Kubelet is listening.",
  },
}
