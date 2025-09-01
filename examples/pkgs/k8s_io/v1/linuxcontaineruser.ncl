# Module: k8s.io.v1.linuxcontaineruser

{
  # LinuxContainerUser represents user identity information in Linux containers
  LinuxContainerUser = {
    gid | optional | Number | doc "GID is the primary gid initially attached to the first process in the container",
    supplementalGroups | optional | Array Number | doc m%"
SupplementalGroups are the supplemental groups initially attached to the first process in the container
"%,
    uid | optional | Number | doc "UID is the primary uid initially attached to the first process in the container",
  },
}
