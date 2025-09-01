# Module: k8s.io.v1.groupversionfordiscovery

{
  # GroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.
  GroupVersionForDiscovery = {
    groupVersion | String | doc "groupVersion specifies the API group and version in the form \"group/version\"",
    version | String | doc m%"
version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.
"%,
  },
}
