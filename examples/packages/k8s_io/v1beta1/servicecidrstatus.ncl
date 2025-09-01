# Module: k8s.io.v1beta1.servicecidrstatus

{
  # ServiceCIDRStatus describes the current state of the ServiceCIDR.
  ServiceCIDRStatus = {
    conditions | optional | Array Condition | doc m%"
conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR. Current service state
"%,
  },
}
