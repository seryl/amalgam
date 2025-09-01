# Module: k8s.io.v1.scopeselector

let scopedresourceselectorrequirement = import "./scopedresourceselectorrequirement.ncl" in

{
  # A scope selector represents the AND of the selectors represented by the scoped-resource selector requirements.
  ScopeSelector = {
    matchExpressions | optional | Array scopedresourceselectorrequirement.ScopedResourceSelectorRequirement | doc "A list of scope selector requirements by scope of the resources.",
  },
}
