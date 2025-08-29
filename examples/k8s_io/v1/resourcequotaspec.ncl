# Module: k8s.io.v1.resourcequotaspec

let scopeselector = import "./scopeselector.ncl" in

{
  # ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
  ResourceQuotaSpec = {
    hard | optional | { .. } | doc m%"
hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
"%,
    scopeSelector | optional | scopeselector.ScopeSelector | doc m%"
scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
"%,
    scopes | optional | Array String | doc m%"
A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
"%,
  },
}
