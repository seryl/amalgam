# Module: k8s.io.authentication.v1

let metav1 = import "../../apimachinery.pkg.apis/meta/v1/mod.ncl" in
let time = metav1.Time in
let objectMeta = metav1.ObjectMeta in

{
  BoundObjectReference = {
    apiVersion
       | String
       | doc "API version of the referent."
       | optional,
    kind
       | String
       | doc "Kind of the referent. Valid kinds are 'Pod' and 'Secret'."
       | optional,
    name
       | String
       | doc "Name of the referent."
       | optional,
    uid
       | String
       | doc "UID of the referent."
       | optional
  },

  SelfSubjectReview = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    status
       | SelfSubjectReviewStatus
       | doc "Status is filled in by the server with the user attributes."
       | optional
  },

  SelfSubjectReviewStatus = {
    userInfo
       | UserInfo
       | doc "User attributes of the user making this request."
       | optional
  },

  TokenRequest = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | TokenRequestSpec
       | doc "Spec holds information about the request being evaluated",
    status
       | TokenRequestStatus
       | doc m%"
Status is filled in by the server and indicates whether the token can be authenticated.
"%
       | optional
  },

  TokenRequestSpec = {
    audiences
       | Array String
       | doc m%"
Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.
"%,
    boundObjectRef
       | BoundObjectReference
       | doc m%"
BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
"%
       | optional,
    expirationSeconds
       | Number
       | doc m%"
ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response.
"%
       | optional
  },

  TokenRequestStatus = {
    expirationTimestamp
       | time
       | doc "ExpirationTimestamp is the time of expiration of the returned token.",
    token
       | String
       | doc "Token is the opaque bearer token."
  },

  TokenReview = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | objectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | TokenReviewSpec
       | doc "Spec holds information about the request being evaluated",
    status
       | TokenReviewStatus
       | doc m%"
Status is filled in by the server and indicates whether the request can be authenticated.
"%
       | optional
  },

  TokenReviewSpec = {
    audiences
       | Array String
       | doc m%"
Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.
"%
       | optional,
    token
       | String
       | doc "Token is the opaque bearer token."
       | optional
  },

  TokenReviewStatus = {
    audiences
       | Array String
       | doc m%"
Audiences are audience identifiers chosen by the authenticator that are compatible with both the TokenReview and token. An identifier is any identifier in the intersection of the TokenReviewSpec audiences and the token's audiences. A client of the TokenReview API that sets the spec.audiences field should validate that a compatible audience identifier is returned in the status.audiences field to ensure that the TokenReview server is audience aware. If a TokenReview returns an empty status.audience field where status.authenticated is "true", the token is valid against the audience of the Kubernetes API server.
"%
       | optional,
    authenticated
       | Bool
       | doc "Authenticated indicates that the token was associated with a known user."
       | optional,
    error
       | String
       | doc "Error indicates that the token couldn't be checked"
       | optional,
    user
       | UserInfo
       | doc "User is the UserInfo associated with the provided token."
       | optional
  },

  UserInfo = {
    extra
       | { .. }
       | doc "Any additional information provided by the authenticator."
       | optional,
    groups
       | Array String
       | doc "The names of groups this user is a part of."
       | optional,
    uid
       | String
       | doc m%"
A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.
"%
       | optional,
    username
       | String
       | doc "The name that uniquely identifies this user among all active users."
       | optional
  }
}
