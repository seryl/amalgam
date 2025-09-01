# Module: k8s.io.v1.resourcestatus

let resourcehealth = import "./resourcehealth.ncl" in

{
  ResourceStatus = {
    name | String | doc m%"
Name of the resource. Must be unique within the pod and match one of the resources from the pod spec.
"%,
    resources | optional | Array resourcehealth.ResourceHealth | doc m%"
List of unique Resources health. Each element in the list contains an unique resource ID and resource health. At a minimum, ResourceID must uniquely identify the Resource allocated to the Pod on the Node for the lifetime of a Pod. See ResourceID type for it's definition.
"%,
  },
}
