# Module: k8s.io.v1.ipaddressspec

let parentreference = import "./parentreference.ncl" in

{
  # IPAddressSpec describe the attributes in an IP Address.
  IPAddressSpec = {
    parentRef | parentreference.ParentReference | doc m%"
ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.
"%,
  },
}
