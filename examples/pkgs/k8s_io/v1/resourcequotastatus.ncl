# Module: k8s.io.v1.resourcequotastatus

{
  # ResourceQuotaStatus defines the enforced hard limits and observed use.
  ResourceQuotaStatus = {
    hard | optional | { .. } | doc m%"
Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
"%,
    used | optional | { .. } | doc "Used is the current observed total usage of the resource in the namespace.",
  },
}
