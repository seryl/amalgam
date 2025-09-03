let clientipconfig = import "./clientipconfig.ncl" in

# Module: k8s.io.v1

# SessionAffinityConfig represents the configurations of session affinity.
{
  clientIP | optional | clientipconfig | doc "clientIP contains the configurations of Client IP based session affinity."
}