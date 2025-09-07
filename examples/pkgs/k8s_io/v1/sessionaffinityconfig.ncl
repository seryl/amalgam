let clientIPConfig = import "./ClientIPConfig.ncl" in

# Module: k8s.io.v1

# SessionAffinityConfig represents the configurations of session affinity.
{
  clientIP
     | clientIPConfig
     | doc "clientIP contains the configurations of Client IP based session affinity."
     | optional
}