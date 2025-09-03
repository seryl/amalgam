# Module: k8s.io.v1alpha1

# Mutation specifies the CEL expression which is used to apply the Mutation.
{
  applyConfiguration | optional | applyconfiguration.ApplyConfiguration | doc m%"
applyConfiguration defines the desired configuration values of an object. The configuration is applied to the admission object using [structured merge diff](https://github.com/kubernetes-sigs/structured-merge-diff). A CEL expression is used to create apply configuration.
"%,
  jsonPatch | optional | jsonpatch.JSONPatch | doc m%"
jsonPatch defines a [JSON patch](https://jsonpatch.com/) operation to perform a mutation to the object. A CEL expression is used to create the JSON patch.
"%,
  patchType | optional | String | doc m%"
patchType indicates the patch strategy used. Allowed values are "ApplyConfiguration" and "JSONPatch". Required.
"%
}
