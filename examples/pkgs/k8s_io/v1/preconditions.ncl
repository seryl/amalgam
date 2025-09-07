# Module: k8s.io.v1

# Preconditions must be fulfilled before an operation (update, delete, etc.) is carried out.
{
  resourceVersion
     | String
     | doc "Specifies the target ResourceVersion"
     | optional,
  uid
     | String
     | doc "Specifies the target UID."
     | optional
}