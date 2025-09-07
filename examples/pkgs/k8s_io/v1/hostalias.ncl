# Module: k8s.io.v1

# HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
{
  hostnames
     | Array String
     | doc "Hostnames for the above IP address."
     | optional,
  ip
     | String
     | doc "IP address of the host file entry."
}