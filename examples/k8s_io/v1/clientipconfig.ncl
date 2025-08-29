# Module: k8s.io.v1.clientipconfig

{
  # ClientIPConfig represents the configurations of Client IP based session affinity.
  ClientIPConfig = {
    timeoutSeconds | optional | Number | doc m%"
timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
"%,
  },
}
