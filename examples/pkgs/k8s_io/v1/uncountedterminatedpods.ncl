# Module: k8s.io.v1

# UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted in Job status counters.
{
  failed
     | Array String
     | doc "failed holds UIDs of failed Pods."
     | optional,
  succeeded
     | Array String
     | doc "succeeded holds UIDs of succeeded Pods."
     | optional
}