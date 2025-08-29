# Module: k8s.io.v1.secretreference

{
  # SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
  SecretReference = {
    name | optional | String | doc "name is unique within a namespace to reference a secret resource.",
    namespace | optional | String | doc "namespace defines the space within which the secret name must be unique.",
  },
}
