let containerResourceMetricStatus = import "./ContainerResourceMetricStatus.ncl" in
let externalMetricStatus = import "./ExternalMetricStatus.ncl" in
let objectMetricStatus = import "./ObjectMetricStatus.ncl" in
let podsMetricStatus = import "./PodsMetricStatus.ncl" in
let resourceMetricStatus = import "./ResourceMetricStatus.ncl" in

# Module: k8s.io.v2

# MetricStatus describes the last-read state of a single metric.
{
  containerResource | optional | containerResourceMetricStatus | doc m%"
container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%,
  external | optional | externalMetricStatus | doc m%"
external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
"%,
  object | optional | objectMetricStatus | doc m%"
object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
"%,
  pods | optional | podsMetricStatus | doc m%"
pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
"%,
  resource | optional | resourceMetricStatus | doc m%"
resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
"%,
  type | String | doc m%"
type is the type of metric source.  It will be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
"%
}