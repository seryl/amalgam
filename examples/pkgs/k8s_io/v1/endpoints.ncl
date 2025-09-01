# Module: k8s.io.v1.endpoints

let endpointsubset = import "./endpointsubset.ncl" in
let objectmeta = import "./objectmeta.ncl" in

{
  # Endpoints is a collection of endpoints that implement the actual service. Example:
  # 
  # 	 Name: "mysvc",
  # 	 Subsets: [
  # 	   {
  # 	     Addresses: [{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}],
  # 	     Ports: [{"name": "a", "port": 8675}, {"name": "b", "port": 309}]
  # 	   },
  # 	   {
  # 	     Addresses: [{"ip": "10.10.3.3"}],
  # 	     Ports: [{"name": "a", "port": 93}, {"name": "b", "port": 76}]
  # 	   },
  # 	]
  # 
  # Endpoints is a legacy API and does not contain information about all Service features. Use discoveryv1.EndpointSlice for complete information about Service endpoints.
  # 
  # Deprecated: This API is deprecated in v1.33+. Use discoveryv1.EndpointSlice.
  Endpoints = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | objectmeta.ObjectMeta | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    subsets | optional | Array endpointsubset.EndpointSubset | doc m%"
The set of all endpoints is the union of all subsets. Addresses are placed into subsets according to the IPs they share. A single address with multiple ports, some of which are ready and some of which are not (because they come from different containers) will result in the address being displayed in different subsets for the different ports. No address will appear in both Addresses and NotReadyAddresses in the same subset. Sets of addresses and ports that comprise a service.
"%,
  },
}
