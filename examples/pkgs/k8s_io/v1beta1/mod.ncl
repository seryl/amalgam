# Kubernetes core v1beta1 types
{
  ServiceCIDRSpec = (import "./servicecidrspec.ncl").ServiceCIDRSpec,
  ServiceCIDR = (import "./servicecidr.ncl").ServiceCIDR,
  VolumeAttributesClass = (import "./volumeattributesclass.ncl").VolumeAttributesClass,
  ServiceCIDRStatus = (import "./servicecidrstatus.ncl").ServiceCIDRStatus,
  IPAddressList = (import "./ipaddresslist.ncl").IPAddressList,
  ParentReference = (import "./parentreference.ncl").ParentReference,
  IPAddress = (import "./ipaddress.ncl").IPAddress,
  IPAddressSpec = (import "./ipaddressspec.ncl").IPAddressSpec,
  VolumeAttributesClassList = (import "./volumeattributesclasslist.ncl").VolumeAttributesClassList,
  ServiceCIDRList = (import "./servicecidrlist.ncl").ServiceCIDRList,
}
