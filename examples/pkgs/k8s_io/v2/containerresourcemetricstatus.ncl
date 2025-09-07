let metricValueStatus = import "./MetricValueStatus.ncl" in

# Module: k8s.io.v2

# ContainerResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing a single container in each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
{
  container
     | String
     | doc "container is the name of the container in the pods of the scaling target",
  current
     | metricValueStatus
     | doc "current contains the current value for the given metric",
  name
     | String
     | doc "name is the name of the resource in question."
}