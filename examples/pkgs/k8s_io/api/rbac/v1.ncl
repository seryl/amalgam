{
  AggregationRule = {
    clusterRoleSelectors
       | Array LabelSelector
       | doc m%"
ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
"%
       | optional
  },

  ClusterRole = {
    aggregationRule
       | io.k8s.api.rbac.v1.AggregationRule
       | doc m%"
AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
"%
       | optional,
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
       | doc "Standard object's metadata."
       | optional,
    rules
       | Array io.k8s.api.rbac.v1.PolicyRule
       | doc "Rules holds all the PolicyRules for this ClusterRole"
       | optional
  },

  ClusterRoleBinding = {
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
       | doc "Standard object's metadata."
       | optional,
    roleRef
       | io.k8s.api.rbac.v1.RoleRef
       | doc m%"
RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error. This field is immutable.
"%,
    subjects
       | Array io.k8s.api.rbac.v1.Subject
       | doc "Subjects holds references to the objects the role applies to."
       | optional
  },

  ClusterRoleBindingList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.rbac.v1.ClusterRoleBinding
       | doc "Items is a list of ClusterRoleBindings",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "Standard object's metadata."
       | optional
  },

  ClusterRoleList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.rbac.v1.ClusterRole
       | doc "Items is a list of ClusterRoles",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "Standard object's metadata."
       | optional
  },

  PolicyRule = {
    apiGroups
       | Array String
       | doc m%"
APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
"%
       | optional,
    nonResourceURLs
       | Array String
       | doc m%"
NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
"%
       | optional,
    resourceNames
       | Array String
       | doc m%"
ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
"%
       | optional,
    resources
       | Array String
       | doc m%"
Resources is a list of resources this rule applies to. '*' represents all resources.
"%
       | optional,
    verbs
       | Array String
       | doc m%"
Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
"%
  },

  Role = {
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
       | doc "Standard object's metadata."
       | optional,
    rules
       | Array io.k8s.api.rbac.v1.PolicyRule
       | doc "Rules holds all the PolicyRules for this Role"
       | optional
  },

  RoleBinding = {
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
       | doc "Standard object's metadata."
       | optional,
    roleRef
       | io.k8s.api.rbac.v1.RoleRef
       | doc m%"
RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error. This field is immutable.
"%,
    subjects
       | Array io.k8s.api.rbac.v1.Subject
       | doc "Subjects holds references to the objects the role applies to."
       | optional
  },

  RoleBindingList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.rbac.v1.RoleBinding
       | doc "Items is a list of RoleBindings",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "Standard object's metadata."
       | optional
  },

  RoleList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.rbac.v1.Role
       | doc "Items is a list of Roles",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "Standard object's metadata."
       | optional
  },

  RoleRef = {
    apiGroup
       | String
       | doc "APIGroup is the group for the resource being referenced",
    kind
       | String
       | doc "Kind is the type of resource being referenced",
    name
       | String
       | doc "Name is the name of resource being referenced"
  },

  Subject = {
    apiGroup
       | String
       | doc m%"
APIGroup holds the API group of the referenced subject. Defaults to "" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
"%
       | optional,
    kind
       | String
       | doc m%"
Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
"%,
    name
       | String
       | doc "Name of the object being referenced.",
    namespace
       | String
       | doc m%"
Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
"%
       | optional
  }
}