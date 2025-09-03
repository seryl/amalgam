let condition = import "../v1/condition.ncl" in

# Module: k8s.io.v1beta1

# ServiceCIDRStatus describes the current state of the ServiceCIDR.
{
  conditions | optional | Array condition | doc m%"
conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR. Current service state
"%
}