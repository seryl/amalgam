# Module: k8s.io.v1.objectfieldselector

{
  # ObjectFieldSelector selects an APIVersioned field of an object.
  ObjectFieldSelector = {
    apiVersion | optional | String | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\".",
    fieldPath | optional | String | doc "Path of the field to select in the specified API version.",
  },
}
