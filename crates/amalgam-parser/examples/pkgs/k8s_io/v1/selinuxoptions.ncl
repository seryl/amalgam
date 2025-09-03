# Module: k8s.io.v1

# SELinuxOptions are the labels to be applied to the container
{
  level | optional | String | doc "Level is SELinux level label that applies to the container.",
  role | optional | String | doc "Role is a SELinux role label that applies to the container.",
  type | optional | String | doc "Type is a SELinux type label that applies to the container.",
  user | optional | String | doc "User is a SELinux user label that applies to the container."
}
