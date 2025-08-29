# Module: k8s.io.v1.sessionaffinityconfig

let clientipconfig = import "./clientipconfig.ncl" in

{
  # SessionAffinityConfig represents the configurations of session affinity.
  SessionAffinityConfig = {
    clientIP | optional | clientipconfig.ClientIPConfig | doc "clientIP contains the configurations of Client IP based session affinity.",
  },
}
