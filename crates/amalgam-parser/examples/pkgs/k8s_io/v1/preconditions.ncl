# Module: k8s.io.v1

# Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
{
  resourceVersion | optional | String | doc "Specifies the target ResourceVersion",
  uid | optional | String | doc "Specifies the target UID."
}
