# Module: k8s.io.v1.tokenrequest

{
  # TokenRequest contains parameters of a service account token.
  TokenRequest = {
    audience | optional | String | doc m%"
audience is the intended audience of the token in "TokenRequestSpec". It will default to the audiences of kube apiserver.
"%,
    expirationSeconds | optional | Number | doc m%"
expirationSeconds is the duration of validity of the token in "TokenRequestSpec". It has the same default value of "ExpirationSeconds" in "TokenRequestSpec".
"%,
  },
}
