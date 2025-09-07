let parentReference = import "./ParentReference.ncl" in

# Module: k8s.io.v1beta1

# IPAddressSpec describe the attributes in an IP Address.
{
  parentRef | parentReference | doc m%"
ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.
"%
}