# UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't been accounted in Job status counters.
{
  failed | optional | Array String | doc "failed holds UIDs of failed Pods.",
  succeeded | optional | Array String | doc "succeeded holds UIDs of succeeded Pods.",
}
