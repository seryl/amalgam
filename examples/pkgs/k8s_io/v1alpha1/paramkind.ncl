# Module: k8s.io.v1alpha1

# ParamKind is a tuple of Group Kind and Version.
{
  apiVersion
     | String
     | doc m%"
APIVersion is the API group version the resources belong to. In format of "group/version". Required.
"%
     | optional,
  kind
     | String
     | doc "Kind is the API kind the resources belong to. Required."
     | optional
}