# Module: k8s.io.v1

# CSINodeSpec holds information about the specification of all CSI drivers installed on a node
{
  drivers | optional | Array csinodedriver.CSINodeDriver | doc m%"
drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
"%
}
