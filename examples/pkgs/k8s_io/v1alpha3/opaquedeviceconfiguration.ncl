let rawextension = import "../v0/rawextension.ncl" in

# Module: k8s.io.v1alpha3

# OpaqueDeviceConfiguration contains configuration parameters for a driver in a format defined by the driver vendor.
{
  driver | optional | String | doc m%"
Driver is used to determine which kubelet plugin needs to be passed these configuration parameters.
An admission policy provided by the driver developer could use this to decide whether it needs to validate them.
Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
"%,
  parameters | optional | rawextension | doc m%"
Parameters can contain arbitrary data. It is the responsibility of the driver developer to handle validation and versioning. Typically this includes self-identification and a version ("kind" + "apiVersion" for Kubernetes types), with conversion between different versions.
The length of the raw data must be smaller or equal to 10 Ki.
"%
}