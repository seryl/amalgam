# Module: k8s.io.v1

# TokenRequest contains parameters of a service account token.
{
  audience
     | String
     | doc m%"
audience is the intended audience of the token in "TokenRequestSpec". It will default to the audiences of kube apiserver.
"%,
  expirationSeconds
     | Number
     | doc m%"
expirationSeconds is the duration of validity of the token in "TokenRequestSpec". It has the same default value of "ExpirationSeconds" in "TokenRequestSpec".
"%
     | optional
}