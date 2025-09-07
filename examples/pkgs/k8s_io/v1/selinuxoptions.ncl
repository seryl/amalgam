# Module: k8s.io.v1

# SELinuxOptions are the labels to be applied to the container
{
  level
     | String
     | doc "Level is SELinux level label that applies to the container."
     | optional,
  role
     | String
     | doc "Role is a SELinux role label that applies to the container."
     | optional,
  type
     | String
     | doc "Type is a SELinux type label that applies to the container."
     | optional,
  user
     | String
     | doc "User is a SELinux user label that applies to the container."
     | optional
}