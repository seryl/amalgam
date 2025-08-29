# Kubernetes core v1beta1 types
{
  ServiceCIDR = (import "./servicecidr.ncl").ServiceCIDR,
  ServiceCIDRStatus = (import "./servicecidrstatus.ncl").ServiceCIDRStatus,
  IPAddressList = (import "./ipaddresslist.ncl").IPAddressList,
  ServiceCIDRList = (import "./servicecidrlist.ncl").ServiceCIDRList,
  ServiceCIDRSpec = (import "./servicecidrspec.ncl").ServiceCIDRSpec,
  IPAddress = (import "./ipaddress.ncl").IPAddress,
  VolumeAttributesClassList = (import "./volumeattributesclasslist.ncl").VolumeAttributesClassList,
  VolumeAttributesClass = (import "./volumeattributesclass.ncl").VolumeAttributesClass,
  ParentReference = (import "./parentreference.ncl").ParentReference,
  IPAddressSpec = (import "./ipaddressspec.ncl").IPAddressSpec,
}
