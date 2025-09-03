# ParamKind is a tuple of Group Kind and Version.
{
  apiVersion | optional | String | doc m%"
APIVersion is the API group version the resources belong to. In format of "group/version". Required.
"%,
  kind | optional | String | doc "Kind is the API kind the resources belong to. Required.",
}
