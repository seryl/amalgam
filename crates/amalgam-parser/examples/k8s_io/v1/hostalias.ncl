# HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
{
  hostnames | optional | Array String | doc "Hostnames for the above IP address.",
  ip | optional | String | doc "IP address of the host file entry.",
}
