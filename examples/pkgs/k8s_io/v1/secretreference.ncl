# Module: k8s.io.v1

# SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
{
  name
     | String
     | doc "name is unique within a namespace to reference a secret resource."
     | optional,
  namespace
     | String
     | doc "namespace defines the space within which the secret name must be unique."
     | optional
}