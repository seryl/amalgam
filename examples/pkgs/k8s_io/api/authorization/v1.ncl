{
  FieldSelectorAttributes = {
    rawSelector
       | String
       | doc m%"
rawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.
"%
       | optional,
    requirements
       | Array FieldSelectorRequirement
       | doc m%"
requirements is the parsed interpretation of a field selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.
"%
       | optional
  },

  LabelSelectorAttributes = {
    rawSelector
       | String
       | doc m%"
rawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.
"%
       | optional,
    requirements
       | Array LabelSelectorRequirement
       | doc m%"
requirements is the parsed interpretation of a label selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.
"%
       | optional
  },

  LocalSubjectAccessReview = {
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
       | ObjectMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.authorization.v1.SubjectAccessReviewSpec
       | doc m%"
Spec holds information about the request being evaluated.  spec.namespace must be equal to the namespace you made the request against.  If empty, it is defaulted.
"%,
    status
       | io.k8s.api.authorization.v1.SubjectAccessReviewStatus
       | doc m%"
Status is filled in by the server and indicates whether the request is allowed or not
"%
       | optional
  },

  NonResourceAttributes = {
    path
       | String
       | doc "Path is the URL path of the request"
       | optional,
    verb
       | String
       | doc "Verb is the standard HTTP verb"
       | optional
  },

  NonResourceRule = {
    nonResourceURLs
       | Array String
       | doc m%"
NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path.  "*" means all.
"%
       | optional,
    verbs
       | Array String
       | doc m%"
Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
"%
  },

  ResourceAttributes = {
    fieldSelector
       | io.k8s.api.authorization.v1.FieldSelectorAttributes
       | doc m%"
fieldSelector describes the limitation on access based on field.  It can only limit access, not broaden it.

This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
"%
       | optional,
    group
       | String
       | doc "Group is the API Group of the Resource.  \"*\" means all."
       | optional,
    labelSelector
       | io.k8s.api.authorization.v1.LabelSelectorAttributes
       | doc m%"
labelSelector describes the limitation on access based on labels.  It can only limit access, not broaden it.

This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
"%
       | optional,
    name
       | String
       | doc m%"
Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
"%
       | optional,
    namespace
       | String
       | doc m%"
Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
"%
       | optional,
    resource
       | String
       | doc "Resource is one of the existing resource types.  \"*\" means all."
       | optional,
    subresource
       | String
       | doc "Subresource is one of the existing resource types.  \"\" means none."
       | optional,
    verb
       | String
       | doc m%"
Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
"%
       | optional,
    version
       | String
       | doc "Version is the API Version of the Resource.  \"*\" means all."
       | optional
  },

  ResourceRule = {
    apiGroups
       | Array String
       | doc m%"
APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
"%
       | optional,
    resourceNames
       | Array String
       | doc m%"
ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
"%
       | optional,
    resources
       | Array String
       | doc m%"
Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
 "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
"%
       | optional,
    verbs
       | Array String
       | doc m%"
Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
"%
  },

  SelfSubjectAccessReview = {
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
       | ObjectMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec
       | doc m%"
Spec holds information about the request being evaluated.  user and groups must be empty
"%,
    status
       | io.k8s.api.authorization.v1.SubjectAccessReviewStatus
       | doc m%"
Status is filled in by the server and indicates whether the request is allowed or not
"%
       | optional
  },

  SelfSubjectAccessReviewSpec = {
    nonResourceAttributes
       | io.k8s.api.authorization.v1.NonResourceAttributes
       | doc "NonResourceAttributes describes information for a non-resource access request"
       | optional,
    resourceAttributes
       | io.k8s.api.authorization.v1.ResourceAttributes
       | doc m%"
ResourceAuthorizationAttributes describes information for a resource access request
"%
       | optional
  },

  SelfSubjectRulesReview = {
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
       | ObjectMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.authorization.v1.SelfSubjectRulesReviewSpec
       | doc "Spec holds information about the request being evaluated.",
    status
       | io.k8s.api.authorization.v1.SubjectRulesReviewStatus
       | doc m%"
Status is filled in by the server and indicates the set of actions a user can perform.
"%
       | optional
  },

  SelfSubjectRulesReviewSpec = {
    namespace
       | String
       | doc "Namespace to evaluate rules for. Required."
       | optional
  },

  SubjectAccessReview = {
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
       | ObjectMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.authorization.v1.SubjectAccessReviewSpec
       | doc "Spec holds information about the request being evaluated",
    status
       | io.k8s.api.authorization.v1.SubjectAccessReviewStatus
       | doc m%"
Status is filled in by the server and indicates whether the request is allowed or not
"%
       | optional
  },

  SubjectAccessReviewSpec = {
    extra
       | { .. }
       | doc m%"
Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
"%
       | optional,
    groups
       | Array String
       | doc "Groups is the groups you're testing for."
       | optional,
    nonResourceAttributes
       | io.k8s.api.authorization.v1.NonResourceAttributes
       | doc "NonResourceAttributes describes information for a non-resource access request"
       | optional,
    resourceAttributes
       | io.k8s.api.authorization.v1.ResourceAttributes
       | doc m%"
ResourceAuthorizationAttributes describes information for a resource access request
"%
       | optional,
    uid
       | String
       | doc "UID information about the requesting user."
       | optional,
    user
       | String
       | doc m%"
User is the user you're testing for. If you specify "User" but not "Groups", then is it interpreted as "What if User were not a member of any groups
"%
       | optional
  },

  SubjectAccessReviewStatus = {
    allowed
       | Bool
       | doc "Allowed is required. True if the action would be allowed, false otherwise.",
    denied
       | Bool
       | doc m%"
Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
"%
       | optional,
    evaluationError
       | String
       | doc m%"
EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
"%
       | optional,
    reason
       | String
       | doc "Reason is optional.  It indicates why a request was allowed or denied."
       | optional
  },

  SubjectRulesReviewStatus = {
    evaluationError
       | String
       | doc m%"
EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
"%
       | optional,
    incomplete
       | Bool
       | doc m%"
Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
"%,
    nonResourceRules
       | Array io.k8s.api.authorization.v1.NonResourceRule
       | doc m%"
NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
"%,
    resourceRules
       | Array io.k8s.api.authorization.v1.ResourceRule
       | doc m%"
ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
"%
  }
}