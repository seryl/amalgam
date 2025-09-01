# Kubernetes core v1beta1 types
{
  IPAddressSpec = (import "./ipaddressspec.ncl").IPAddressSpec,
  IPAddress = (import "./ipaddress.ncl").IPAddress,
  ServiceCIDR = (import "./servicecidr.ncl").ServiceCIDR,
  VolumeAttributesClass = (import "./volumeattributesclass.ncl").VolumeAttributesClass,
  ServiceCIDRStatus = (import "./servicecidrstatus.ncl").ServiceCIDRStatus,
  ParentReference = (import "./parentreference.ncl").ParentReference,
  ServiceCIDRSpec = (import "./servicecidrspec.ncl").ServiceCIDRSpec,
}
