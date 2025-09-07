let resourceStatus = import "./ResourceStatus.ncl" in
let containerState = import "./ContainerState.ncl" in
let resourceRequirements = import "./ResourceRequirements.ncl" in
let containerUser = import "./ContainerUser.ncl" in
let volumeMountStatus = import "./VolumeMountStatus.ncl" in

# Module: k8s.io.v1

# ContainerStatus contains details for the current status of this container.
{
  allocatedResources
     | { .. }
     | doc m%"
AllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.
"%
     | optional,
  allocatedResourcesStatus
     | Array resourceStatus
     | doc m%"
AllocatedResourcesStatus represents the status of various resources allocated for this Pod.
"%
     | optional,
  containerID
     | String
     | doc m%"
ContainerID is the ID of the container in the format '<type>://<container_id>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example "containerd").
"%
     | optional,
  image
     | String
     | doc m%"
Image is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.
"%,
  imageID
     | String
     | doc m%"
ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.
"%,
  lastState
     | containerState
     | doc m%"
LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.
"%
     | optional,
  name
     | String
     | doc m%"
Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.
"%,
  ready
     | Bool
     | doc m%"
Ready specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).
The value is typically used to determine whether a container is ready to accept traffic.
"%,
  resources
     | resourceRequirements
     | doc m%"
Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.
"%
     | optional,
  restartCount
     | Number
     | doc m%"
RestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.
"%,
  started
     | Bool
     | doc m%"
Started indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.
"%
     | optional,
  state
     | containerState
     | doc "State holds details about the container's current condition."
     | optional,
  stopSignal
     | String
     | doc "StopSignal reports the effective stop signal for this container"
     | optional,
  user
     | containerUser
     | doc m%"
User represents user identity information initially attached to the first process of the container
"%
     | optional,
  volumeMounts
     | Array volumeMountStatus
     | doc "Status of volume mounts."
     | optional
}