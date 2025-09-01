# Module: k8s.io.v1.servicecidrstatus

let condition = import "./condition.ncl" in

{
  # ServiceCIDRStatus describes the current state of the ServiceCIDR.
  ServiceCIDRStatus = {
    conditions | optional | Array condition.Condition | doc m%"
conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR. Current service state
"%,
  },
}
