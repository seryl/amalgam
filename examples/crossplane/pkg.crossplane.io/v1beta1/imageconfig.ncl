# Module: imageconfig.pkg.crossplane.io

let k8s_io_objectmeta = import "../../../k8s_io/v1/objectmeta.ncl" in

{
  ImageConfig = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | k8s_io_objectmeta.ObjectMeta,
    spec | optional | {
      matchImages | Array {
        prefix | String | doc m%"
Prefix is the prefix that should be matched. When multiple prefix rules
match an image path, the longest one takes precedence.
"%,
        type | String | doc "Type is the type of match." | default = "Prefix",
      } | doc m%"
MatchImages is a list of image matching rules. This ImageConfig will
match an image if any one of these rules is satisfied. In the case where
multiple ImageConfigs match an image for a given purpose the one with the
most specific match will be used. If multiple rules of equal specificity
match an arbitrary one will be selected.
"%,
      registry | optional | {
        authentication | optional | {
          pullSecretRef | {
            name | String | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"% | default = "",
          } | doc m%"
PullSecretRef is a reference to a secret that contains the credentials for
the registry.
"%,
        } | doc "Authentication is the authentication information for the registry.",
      } | doc "Registry is the configuration for the registry.",
      rewriteImage | optional | {
        prefix | String | doc m%"
Prefix is the prefix that will replace the portion of the image's path
matched by the prefix in the ImageMatch. If multiple prefixes matched,
the longest one will be replaced.
"%,
      } | doc "RewriteImage defines how a matched image's path should be rewritten.",
      verification | optional | {
        cosign | optional | {
          authorities | Array {
            attestations | optional | Array {
              name | String | doc "Name of the attestation.",
              predicateType | String | doc m%"
PredicateType defines which predicate type to verify. Matches cosign
verify-attestation options.
"%,
            } | doc m%"
Attestations is a list of individual attestations for this authority,
once the signature for this authority has been verified.
"%,
            key | optional | {
              hashAlgorithm | String | doc m%"
HashAlgorithm always defaults to sha256 if the algorithm hasn't been explicitly set
"% | default = "sha256",
              secretRef | {
                key | String | doc "The key to select.",
                name | String | doc "Name of the secret.",
              } | doc "SecretRef sets a reference to a secret with the key.",
            } | doc "Key defines the type of key to validate the image.",
            keyless | optional | {
              identities | Array {
                issuer | optional | String | doc "Issuer defines the issuer for this identity.",
                issuerRegExp | optional | String | doc m%"
IssuerRegExp specifies a regular expression to match the issuer for this identity.
This has precedence over the Issuer field.
"%,
                subject | optional | String | doc "Subject defines the subject for this identity.",
                subjectRegExp | optional | String | doc m%"
SubjectRegExp specifies a regular expression to match the subject for this identity.
This has precedence over the Subject field.
"%,
              } | doc "Identities sets a list of identities.",
              insecureIgnoreSCT | optional | Bool | doc "InsecureIgnoreSCT omits verifying if a certificate contains an embedded SCT",
            } | doc m%"
Keyless sets the configuration to verify the authority against a Fulcio
instance.
"%,
            name | String | doc "Name is the name for this authority.",
          } | doc "Authorities defines the rules for discovering and validating signatures.",
        } | doc "Cosign is the configuration for verifying the image using cosign.",
        provider | String | doc "Provider is the provider that should be used to verify the image.",
      } | doc "Verification contains the configuration for verifying the image.",
    } | doc "ImageConfigSpec contains the configuration for matching images.",
  },
}
