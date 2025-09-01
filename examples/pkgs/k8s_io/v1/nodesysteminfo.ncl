# Module: k8s.io.v1.nodesysteminfo

let nodeswapstatus = import "./nodeswapstatus.ncl" in

{
  # NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
  NodeSystemInfo = {
    architecture | String | doc "The Architecture reported by the node",
    bootID | String | doc "Boot ID reported by the node.",
    containerRuntimeVersion | String | doc m%"
ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
"%,
    kernelVersion | String | doc "Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).",
    kubeProxyVersion | String | doc "Deprecated: KubeProxy Version reported by the node.",
    kubeletVersion | String | doc "Kubelet Version reported by the node.",
    machineID | String | doc m%"
MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
"%,
    operatingSystem | String | doc "The Operating System reported by the node",
    osImage | String | doc m%"
OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
"%,
    swap | optional | nodeswapstatus.NodeSwapStatus | doc "Swap Info reported by the node.",
    systemUUID | String | doc m%"
SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid
"%,
  },
}
