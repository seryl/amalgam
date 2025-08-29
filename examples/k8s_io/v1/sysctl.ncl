# Module: k8s.io.v1.sysctl

{
  # Sysctl defines a kernel parameter to be set
  Sysctl = {
    name | String | doc "Name of a property to set",
    value | String | doc "Value of a property to set",
  },
}
