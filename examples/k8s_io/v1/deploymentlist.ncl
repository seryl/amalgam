# Module: k8s.io.v1.deploymentlist

let listmeta = import "./listmeta.ncl" in
let deployment = import "./deployment.ncl" in

{
  # DeploymentList is a list of Deployments.
  DeploymentList = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    items | Array deployment.Deployment | doc "Items is the list of Deployments.",
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | listmeta.ListMeta | doc "Standard list metadata.",
  },
}
