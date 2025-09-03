let parentreference = import "./parentreference.ncl" in

# Module: k8s.io.v1beta1

# IPAddressSpec describe the attributes in an IP Address.
{
  parentRef | optional | parentreference | doc m%"
ParentRef references the resource that an IPAddress is attached to. An IPAddress must reference a parent object.
"%
}