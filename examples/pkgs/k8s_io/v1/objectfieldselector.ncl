# Module: k8s.io.v1

# ObjectFieldSelector selects an APIVersioned field of an object.
{
  apiVersion | optional | String | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\".",
  fieldPath | String | doc "Path of the field to select in the specified API version."
}