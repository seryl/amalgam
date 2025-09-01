# Module: k8s.io.v1.serveraddressbyclientcidr

{
  # ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.
  ServerAddressByClientCIDR = {
    clientCIDR | String | doc m%"
The CIDR with which clients can match their IP to figure out the server address that they should use.
"%,
    serverAddress | String | doc m%"
Address of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.
"%,
  },
}
