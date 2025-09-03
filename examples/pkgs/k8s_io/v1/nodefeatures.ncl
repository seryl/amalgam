# Module: k8s.io.v1

# NodeFeatures describes the set of features implemented by the CRI implementation. The features contained in the NodeFeatures should depend only on the cri implementation independent of runtime handlers.
{
  supplementalGroupsPolicy | optional | Bool | doc m%"
SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
"%
}