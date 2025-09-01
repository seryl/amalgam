# Kubernetes core v1beta1 types
{
  IPAddressList = (import "./ipaddresslist.ncl").IPAddressList,
  VolumeAttributesClass = (import "./volumeattributesclass.ncl").VolumeAttributesClass,
  ServiceCIDRSpec = (import "./servicecidrspec.ncl").ServiceCIDRSpec,
  IPAddress = (import "./ipaddress.ncl").IPAddress,
  ServiceCIDRList = (import "./servicecidrlist.ncl").ServiceCIDRList,
  VolumeAttributesClassList = (import "./volumeattributesclasslist.ncl").VolumeAttributesClassList,
  IPAddressSpec = (import "./ipaddressspec.ncl").IPAddressSpec,
  ServiceCIDR = (import "./servicecidr.ncl").ServiceCIDR,
  ParentReference = (import "./parentreference.ncl").ParentReference,
  ServiceCIDRStatus = (import "./servicecidrstatus.ncl").ServiceCIDRStatus,
}
