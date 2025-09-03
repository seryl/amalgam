# Module: k8s.io.v1

# ContainerUser represents user identity information
{
  linux | optional | linuxcontaineruser.LinuxContainerUser | doc m%"
Linux holds user identity information initially attached to the first process of the containers in Linux. Note that the actual running identity can be changed if the process has enough privilege to do so.
"%
}
