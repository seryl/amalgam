let objectMeta = import "../../k8s_io/v1/ObjectMeta.ncl" in
let k8s_io_api_core_v1_EnvVar = import "./k8s.io/api/core/v1.EnvVar.ncl" in
let k8s_io_api_core_v1_EnvFromSource = import "./k8s.io/api/core/v1.EnvFromSource.ncl" in
let k8s_io_api_core_v1_VolumeMount = import "./k8s.io/api/core/v1.VolumeMount.ncl" in
let k8s_io_api_core_v1_Toleration = import "./k8s.io/api/core/v1.Toleration.ncl" in


{
  apiVersion
     | String
     | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
     | optional,
  kind
     | String
     | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
     | optional,
  metadata
     | objectMeta.ObjectMeta
     | optional,
  spec
     | {
    deploymentTemplate
       | {
      metadata
         | {
        annotations
           | {
        }
           | doc m%"
Annotations is an unstructured key value map stored with a resource that
may be set by external tools to store and retrieve arbitrary metadata.
They are not queryable and should be preserved when modifying objects.
More info: http:https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/
"%
           | optional,
        labels
           | {
        }
           | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. Labels will be merged with internal labels
used by crossplane, and labels with a crossplane.io key might be
overwritten.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%
           | optional,
        name
           | String
           | doc "Name is the name of the object."
           | optional
      }
         | doc "Metadata contains the configurable metadata fields for the Deployment."
         | optional,
      spec
         | {
        minReadySeconds
           | Number
           | doc m%"
Minimum number of seconds for which a newly created pod should be ready
without any of its container crashing, for it to be considered available.
Defaults to 0 (pod will be considered available as soon as it is ready)
"%
           | optional,
        paused
           | Bool
           | doc "Indicates that the deployment is paused."
           | optional,
        progressDeadlineSeconds
           | Number
           | doc m%"
The maximum time in seconds for a deployment to make progress before it
is considered to be failed. The deployment controller will continue to
process failed deployments and a condition with a ProgressDeadlineExceeded
reason will be surfaced in the deployment status. Note that progress will
not be estimated during the time a deployment is paused. Defaults to 600s.
"%
           | optional,
        replicas
           | Number
           | doc m%"
Number of desired pods. This is a pointer to distinguish between explicit
zero and not specified. Defaults to 1.
"%
           | optional,
        revisionHistoryLimit
           | Number
           | doc m%"
The number of old ReplicaSets to retain to allow rollback.
This is a pointer to distinguish between explicit zero and not specified.
Defaults to 10.
"%
           | optional,
        selector
           | {
          matchExpressions
             | Array {
            key
               | String
               | doc "key is the label key that the selector applies to.",
            operator
               | String
               | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
            values
               | Array String
               | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
               | optional
          }
             | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
             | optional,
          matchLabels
             | {
          }
             | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
             | optional
        }
           | doc m%"
Label selector for pods. Existing ReplicaSets whose pods are
selected by this will be the ones affected by this deployment.
It must match the pod template's labels.
"%,
        strategy
           | {
          rollingUpdate
             | {
            maxSurge
               | Number | String
               | doc m%"
The maximum number of pods that can be scheduled above the desired number of
pods.
Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
This can not be 0 if MaxUnavailable is 0.
Absolute number is calculated from percentage by rounding up.
Defaults to 25%.
Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when
the rolling update starts, such that the total number of old and new pods do not exceed
130% of desired pods. Once old pods have been killed,
new ReplicaSet can be scaled up further, ensuring that total number of pods running
at any time during the update is at most 130% of desired pods.
"%
               | optional,
            maxUnavailable
               | Number | String
               | doc m%"
The maximum number of pods that can be unavailable during the update.
Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
Absolute number is calculated from percentage by rounding down.
This can not be 0 if MaxSurge is 0.
Defaults to 25%.
Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods
immediately when the rolling update starts. Once new pods are ready, old ReplicaSet
can be scaled down further, followed by scaling up the new ReplicaSet, ensuring
that the total number of pods available at all times during the update is at
least 70% of desired pods.
"%
               | optional
          }
             | doc m%"
Rolling update config params. Present only if DeploymentStrategyType =
RollingUpdate.
"%
             | optional,
          type
             | String
             | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%
             | optional
        }
           | doc "The deployment strategy to use to replace existing pods with new ones."
           | optional,
        template
           | {
          metadata
             | {
            annotations
               | {
            }
               | optional,
            finalizers
               | Array String
               | optional,
            labels
               | {
            }
               | optional,
            name
               | String
               | optional,
            namespace
               | String
               | optional
          }
             | doc m%"
Standard object's metadata.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
             | optional,
          spec
             | {
            activeDeadlineSeconds
               | Number
               | doc m%"
Optional duration in seconds the pod may be active on the node relative to
StartTime before the system will actively try to mark it failed and kill associated containers.
Value must be a positive integer.
"%
               | optional,
            affinity
               | {
              nodeAffinity
                 | {
                preferredDuringSchedulingIgnoredDuringExecution
                   | Array {
                  preference
                     | {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "The label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                      values
                         | Array String
                         | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%
                         | optional
                    }
                       | doc "A list of node selector requirements by node's labels."
                       | optional,
                    matchFields
                       | Array {
                      key
                         | String
                         | doc "The label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                      values
                         | Array String
                         | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%
                         | optional
                    }
                       | doc "A list of node selector requirements by node's fields."
                       | optional
                  }
                     | doc "A node selector term, associated with the corresponding weight.",
                  weight
                     | Number
                     | doc m%"
Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
"%
                }
                   | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node matches the corresponding matchExpressions; the
node(s) with the highest sum are the most preferred.
"%
                   | optional,
                requiredDuringSchedulingIgnoredDuringExecution
                   | {
                  nodeSelectorTerms
                     | Array {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "The label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                      values
                         | Array String
                         | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%
                         | optional
                    }
                       | doc "A list of node selector requirements by node's labels."
                       | optional,
                    matchFields
                       | Array {
                      key
                         | String
                         | doc "The label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                      values
                         | Array String
                         | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%
                         | optional
                    }
                       | doc "A list of node selector requirements by node's fields."
                       | optional
                  }
                     | doc "Required. A list of node selector terms. The terms are ORed."
                }
                   | doc m%"
If the affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to an update), the system
may or may not try to eventually evict the pod from its node.
"%
                   | optional
              }
                 | doc "Describes node affinity scheduling rules for the pod."
                 | optional,
              podAffinity
                 | {
                preferredDuringSchedulingIgnoredDuringExecution
                   | Array {
                  podAffinityTerm
                     | {
                    labelSelector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%
                       | optional,
                    matchLabelKeys
                       | Array String
                       | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
"%
                       | optional,
                    mismatchLabelKeys
                       | Array String
                       | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
"%
                       | optional,
                    namespaceSelector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%
                       | optional,
                    namespaces
                       | Array String
                       | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%
                       | optional,
                    topologyKey
                       | String
                       | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%
                  }
                     | doc "Required. A pod affinity term, associated with the corresponding weight.",
                  weight
                     | Number
                     | doc m%"
weight associated with matching the corresponding podAffinityTerm,
in the range 1-100.
"%
                }
                   | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
node(s) with the highest sum are the most preferred.
"%
                   | optional,
                requiredDuringSchedulingIgnoredDuringExecution
                   | Array {
                  labelSelector
                     | {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "key is the label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                      values
                         | Array String
                         | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                         | optional
                    }
                       | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                       | optional,
                    matchLabels
                       | {
                    }
                       | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                       | optional
                  }
                     | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%
                     | optional,
                  matchLabelKeys
                     | Array String
                     | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
"%
                     | optional,
                  mismatchLabelKeys
                     | Array String
                     | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
"%
                     | optional,
                  namespaceSelector
                     | {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "key is the label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                      values
                         | Array String
                         | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                         | optional
                    }
                       | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                       | optional,
                    matchLabels
                       | {
                    }
                       | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                       | optional
                  }
                     | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%
                     | optional,
                  namespaces
                     | Array String
                     | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%
                     | optional,
                  topologyKey
                     | String
                     | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%
                }
                   | doc m%"
If the affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to a pod label update), the
system may or may not try to eventually evict the pod from its node.
When there are multiple elements, the lists of nodes corresponding to each
podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%
                   | optional
              }
                 | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%
                 | optional,
              podAntiAffinity
                 | {
                preferredDuringSchedulingIgnoredDuringExecution
                   | Array {
                  podAffinityTerm
                     | {
                    labelSelector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%
                       | optional,
                    matchLabelKeys
                       | Array String
                       | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
"%
                       | optional,
                    mismatchLabelKeys
                       | Array String
                       | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
"%
                       | optional,
                    namespaceSelector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%
                       | optional,
                    namespaces
                       | Array String
                       | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%
                       | optional,
                    topologyKey
                       | String
                       | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%
                  }
                     | doc "Required. A pod affinity term, associated with the corresponding weight.",
                  weight
                     | Number
                     | doc m%"
weight associated with matching the corresponding podAffinityTerm,
in the range 1-100.
"%
                }
                   | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the anti-affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling anti-affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
node(s) with the highest sum are the most preferred.
"%
                   | optional,
                requiredDuringSchedulingIgnoredDuringExecution
                   | Array {
                  labelSelector
                     | {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "key is the label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                      values
                         | Array String
                         | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                         | optional
                    }
                       | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                       | optional,
                    matchLabels
                       | {
                    }
                       | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                       | optional
                  }
                     | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%
                     | optional,
                  matchLabelKeys
                     | Array String
                     | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
"%
                     | optional,
                  mismatchLabelKeys
                     | Array String
                     | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
"%
                     | optional,
                  namespaceSelector
                     | {
                    matchExpressions
                       | Array {
                      key
                         | String
                         | doc "key is the label key that the selector applies to.",
                      operator
                         | String
                         | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                      values
                         | Array String
                         | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                         | optional
                    }
                       | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                       | optional,
                    matchLabels
                       | {
                    }
                       | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                       | optional
                  }
                     | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%
                     | optional,
                  namespaces
                     | Array String
                     | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%
                     | optional,
                  topologyKey
                     | String
                     | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%
                }
                   | doc m%"
If the anti-affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the anti-affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to a pod label update), the
system may or may not try to eventually evict the pod from its node.
When there are multiple elements, the lists of nodes corresponding to each
podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%
                   | optional
              }
                 | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%
                 | optional
            }
               | doc "If specified, the pod's scheduling constraints"
               | optional,
            automountServiceAccountToken
               | Bool
               | doc m%"
AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
"%
               | optional,
            containers
               | Array {
              args
                 | Array String
                 | doc m%"
Arguments to the entrypoint.
The container image's CMD is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              command
                 | Array String
                 | doc m%"
Entrypoint array. Not executed within a shell.
The container image's ENTRYPOINT is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              env
                 | Array k8s_io_api_core_v1_EnvVar.EnvVar
                 | doc m%"
List of environment variables to set in the container.
Cannot be updated.
"%
                 | optional,
              envFrom
                 | Array k8s_io_api_core_v1_EnvFromSource.EnvFromSource
                 | doc m%"
List of sources to populate environment variables in the container.
The keys defined within a source must be a C_IDENTIFIER. All invalid keys
will be reported as an event when the container is starting. When a key exists in multiple
sources, the value associated with the last source will take precedence.
Values defined by an Env with a duplicate key will take precedence.
Cannot be updated.
"%
                 | optional,
              image
                 | String
                 | doc m%"
Container image name.
More info: https://kubernetes.io/docs/concepts/containers/images
This field is optional to allow higher level config management to default or override
container images in workload controllers like Deployments and StatefulSets.
"%
                 | optional,
              imagePullPolicy
                 | String
                 | doc m%"
Image pull policy.
One of Always, Never, IfNotPresent.
Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%
                 | optional,
              lifecycle
                 | {
                postStart
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PostStart is called immediately after a container is created. If the handler fails,
the container is terminated and restarted according to its restart policy.
Other management of the container blocks until the hook completes.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                preStop
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PreStop is called immediately before a container is terminated due to an
API request or management event such as liveness/startup probe failure,
preemption, resource contention, etc. The handler is not called if the
container crashes or exits. The Pod's termination grace period countdown begins before the
PreStop hook is executed. Regardless of the outcome of the handler, the
container will eventually terminate within the Pod's termination grace
period (unless delayed by finalizers). Other management of the container blocks until the hook completes
or until the termination grace period is reached.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                stopSignal
                   | String
                   | doc m%"
StopSignal defines which signal will be sent to a container when it is being stopped.
If not specified, the default is defined by the container runtime in use.
StopSignal can only be set for Pods with a non-empty .spec.os.name
"%
                   | optional
              }
                 | doc m%"
Actions that the management system should take in response to container lifecycle events.
Cannot be updated.
"%
                 | optional,
              livenessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
Periodic probe of container liveness.
Container will be restarted if the probe fails.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              name
                 | String
                 | doc m%"
Name of the container specified as a DNS_LABEL.
Each container in a pod must have a unique name (DNS_LABEL).
Cannot be updated.
"%,
              ports
                 | Array {
                containerPort
                   | Number
                   | doc m%"
Number of port to expose on the pod's IP address.
This must be a valid port number, 0 < x < 65536.
"%,
                hostIP
                   | String
                   | doc "What host IP to bind the external port to."
                   | optional,
                hostPort
                   | Number
                   | doc m%"
Number of port to expose on the host.
If specified, this must be a valid port number, 0 < x < 65536.
If HostNetwork is specified, this must match ContainerPort.
Most containers do not need this.
"%
                   | optional,
                name
                   | String
                   | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each
named port in a pod must have a unique name. Name for the port that can be
referred to by services.
"%
                   | optional,
                protocol
                   | String
                   | doc m%"
Protocol for port. Must be UDP, TCP, or SCTP.
Defaults to "TCP".
"%
                   = "TCP"
              }
                 | doc m%"
List of ports to expose from the container. Not specifying a port here
DOES NOT prevent that port from being exposed. Any port which is
listening on the default "0.0.0.0" address inside a container will be
accessible from the network.
Modifying this array with strategic merge patch may corrupt the data.
For more information See https://github.com/kubernetes/kubernetes/issues/108255.
Cannot be updated.
"%
                 | optional,
              readinessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
Periodic probe of container service readiness.
Container will be removed from service endpoints if the probe fails.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              resizePolicy
                 | Array {
                resourceName
                   | String
                   | doc m%"
Name of the resource to which this resource resize policy applies.
Supported values: cpu, memory.
"%,
                restartPolicy
                   | String
                   | doc m%"
Restart policy to apply when specified resource is resized.
If not specified, it defaults to NotRequired.
"%
              }
                 | doc "Resources resize policy for the container."
                 | optional,
              resources
                 | {
                claims
                   | Array {
                  name
                     | String
                     | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of
the Pod where this field is used. It makes that resource available
inside a container.
"%,
                  request
                     | String
                     | doc m%"
Request is the name chosen for a request in the referenced claim.
If empty, everything from the claim is made available, otherwise
only the result of this request.
"%
                     | optional
                }
                   | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims,
that are used by this container.
This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.
This field is immutable. It can only be set for containers.
"%
                   | optional,
                limits
                   | {
                }
                   | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional,
                requests
                   | {
                }
                   | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional
              }
                 | doc m%"
Compute Resources required by this container.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                 | optional,
              restartPolicy
                 | String
                 | doc m%"
RestartPolicy defines the restart behavior of individual containers in a pod.
This field may only be set for init containers, and the only allowed value is "Always".
For non-init containers or when this field is not specified,
the restart behavior is defined by the Pod's restart policy and the container type.
Setting the RestartPolicy as "Always" for the init container will have the following effect:
this init container will be continually restarted on
exit until all regular containers have terminated. Once all regular
containers have completed, all init containers with restartPolicy "Always"
will be shut down. This lifecycle differs from normal init containers and
is often referred to as a "sidecar" container. Although this init
container still starts in the init container sequence, it does not wait
for the container to complete before proceeding to the next init
container. Instead, the next init container starts immediately after this
init container is started, or after any startupProbe has successfully
completed.
"%
                 | optional,
              securityContext
                 | {
                allowPrivilegeEscalation
                   | Bool
                   | doc m%"
AllowPrivilegeEscalation controls whether a process can gain more
privileges than its parent process. This bool directly controls if
the no_new_privs flag will be set on the container process.
AllowPrivilegeEscalation is true always when the container is:
1) run as Privileged
2) has CAP_SYS_ADMIN
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                appArmorProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%
                }
                   | doc m%"
appArmorProfile is the AppArmor options to use by this container. If set, this profile
overrides the pod's appArmorProfile.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                capabilities
                   | {
                  add
                     | Array String
                     | doc "Added capabilities"
                     | optional,
                  drop
                     | Array String
                     | doc "Removed capabilities"
                     | optional
                }
                   | doc m%"
The capabilities to add/drop when running containers.
Defaults to the default set of capabilities granted by the container runtime.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                privileged
                   | Bool
                   | doc m%"
Run container in privileged mode.
Processes in privileged containers are essentially equivalent to root on the host.
Defaults to false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                procMount
                   | String
                   | doc m%"
procMount denotes the type of proc mount to use for the containers.
The default value is Default which uses the container runtime defaults for
readonly paths and masked paths.
This requires the ProcMountType feature flag to be enabled.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                readOnlyRootFilesystem
                   | Bool
                   | doc m%"
Whether this container has a read-only root filesystem.
Default is false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsGroup
                   | Number
                   | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsNonRoot
                   | Bool
                   | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                   | optional,
                runAsUser
                   | Number
                   | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seLinuxOptions
                   | {
                  level
                     | String
                     | doc "Level is SELinux level label that applies to the container."
                     | optional,
                  role
                     | String
                     | doc "Role is a SELinux role label that applies to the container."
                     | optional,
                  type
                     | String
                     | doc "Type is a SELinux type label that applies to the container."
                     | optional,
                  user
                     | String
                     | doc "User is a SELinux user label that applies to the container."
                     | optional
                }
                   | doc m%"
The SELinux context to be applied to the container.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seccompProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:
Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%
                }
                   | doc m%"
The seccomp options to use by this container. If seccomp options are
provided at both the pod & container level, the container options
override the pod options.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                windowsOptions
                   | {
                  gmsaCredentialSpec
                     | String
                     | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%
                     | optional,
                  gmsaCredentialSpecName
                     | String
                     | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use."
                     | optional,
                  hostProcess
                     | Bool
                     | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%
                     | optional,
                  runAsUserName
                     | String
                     | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                     | optional
                }
                   | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options from the PodSecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%
                   | optional
              }
                 | doc m%"
SecurityContext defines the security options the container should be run with.
If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
"%
                 | optional,
              startupProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
StartupProbe indicates that the Pod has successfully initialized.
If specified, no other probes are executed until this completes successfully.
If this probe fails, the Pod will be restarted, just as if the livenessProbe failed.
This can be used to provide different probe parameters at the beginning of a Pod's lifecycle,
when it might take a long time to load data or warm a cache, than during steady-state operation.
This cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              stdin
                 | Bool
                 | doc m%"
Whether this container should allocate a buffer for stdin in the container runtime. If this
is not set, reads from stdin in the container will always result in EOF.
Default is false.
"%
                 | optional,
              stdinOnce
                 | Bool
                 | doc m%"
Whether the container runtime should close the stdin channel after it has been opened by
a single attach. When stdin is true the stdin stream will remain open across multiple attach
sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the
first client attaches to stdin, and then remains open and accepts data until the client disconnects,
at which time stdin is closed and remains closed until the container is restarted. If this
flag is false, a container processes that reads from stdin will never receive an EOF.
Default is false
"%
                 | optional,
              terminationMessagePath
                 | String
                 | doc m%"
Optional: Path at which the file to which the container's termination message
will be written is mounted into the container's filesystem.
Message written is intended to be brief final status, such as an assertion failure message.
Will be truncated by the node if greater than 4096 bytes. The total message length across
all containers will be limited to 12kb.
Defaults to /dev/termination-log.
Cannot be updated.
"%
                 | optional,
              terminationMessagePolicy
                 | String
                 | doc m%"
Indicate how the termination message should be populated. File will use the contents of
terminationMessagePath to populate the container status message on both success and failure.
FallbackToLogsOnError will use the last chunk of container log output if the termination
message file is empty and the container exited with an error.
The log output is limited to 2048 bytes or 80 lines, whichever is smaller.
Defaults to File.
Cannot be updated.
"%
                 | optional,
              tty
                 | Bool
                 | doc m%"
Whether this container should allocate a TTY for itself, also requires 'stdin' to be true.
Default is false.
"%
                 | optional,
              volumeDevices
                 | Array {
                devicePath
                   | String
                   | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
                name
                   | String
                   | doc "name must match the name of a persistentVolumeClaim in the pod"
              }
                 | doc "volumeDevices is the list of block devices to be used by the container."
                 | optional,
              volumeMounts
                 | Array k8s_io_api_core_v1_VolumeMount.VolumeMount
                 | doc m%"
Pod volumes to mount into the container's filesystem.
Cannot be updated.
"%
                 | optional,
              workingDir
                 | String
                 | doc m%"
Container's working directory.
If not specified, the container runtime's default will be used, which
might be configured in the container image.
Cannot be updated.
"%
                 | optional
            }
               | doc m%"
List of containers belonging to the pod.
Containers cannot currently be added or removed.
There must be at least one container in a Pod.
Cannot be updated.
"%,
            dnsConfig
               | {
              nameservers
                 | Array String
                 | doc m%"
A list of DNS name server IP addresses.
This will be appended to the base nameservers generated from DNSPolicy.
Duplicated nameservers will be removed.
"%
                 | optional,
              options
                 | Array {
                name
                   | String
                   | doc m%"
Name is this DNS resolver option's name.
Required.
"%
                   | optional,
                value
                   | String
                   | doc "Value is this DNS resolver option's value."
                   | optional
              }
                 | doc m%"
A list of DNS resolver options.
This will be merged with the base options generated from DNSPolicy.
Duplicated entries will be removed. Resolution options given in Options
will override those that appear in the base DNSPolicy.
"%
                 | optional,
              searches
                 | Array String
                 | doc m%"
A list of DNS search domains for host-name lookup.
This will be appended to the base search paths generated from DNSPolicy.
Duplicated search paths will be removed.
"%
                 | optional
            }
               | doc m%"
Specifies the DNS parameters of a pod.
Parameters specified here will be merged to the generated DNS
configuration based on DNSPolicy.
"%
               | optional,
            dnsPolicy
               | String
               | doc m%"
Set DNS policy for the pod.
Defaults to "ClusterFirst".
Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'.
DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy.
To have DNS options set along with hostNetwork, you have to specify DNS policy
explicitly to 'ClusterFirstWithHostNet'.
"%
               | optional,
            enableServiceLinks
               | Bool
               | doc m%"
EnableServiceLinks indicates whether information about services should be injected into pod's
environment variables, matching the syntax of Docker links.
Optional: Defaults to true.
"%
               | optional,
            ephemeralContainers
               | Array {
              args
                 | Array String
                 | doc m%"
Arguments to the entrypoint.
The image's CMD is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              command
                 | Array String
                 | doc m%"
Entrypoint array. Not executed within a shell.
The image's ENTRYPOINT is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              env
                 | Array k8s_io_api_core_v1_EnvVar.EnvVar
                 | doc m%"
List of environment variables to set in the container.
Cannot be updated.
"%
                 | optional,
              envFrom
                 | Array k8s_io_api_core_v1_EnvFromSource.EnvFromSource
                 | doc m%"
List of sources to populate environment variables in the container.
The keys defined within a source must be a C_IDENTIFIER. All invalid keys
will be reported as an event when the container is starting. When a key exists in multiple
sources, the value associated with the last source will take precedence.
Values defined by an Env with a duplicate key will take precedence.
Cannot be updated.
"%
                 | optional,
              image
                 | String
                 | doc m%"
Container image name.
More info: https://kubernetes.io/docs/concepts/containers/images
"%
                 | optional,
              imagePullPolicy
                 | String
                 | doc m%"
Image pull policy.
One of Always, Never, IfNotPresent.
Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%
                 | optional,
              lifecycle
                 | {
                postStart
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PostStart is called immediately after a container is created. If the handler fails,
the container is terminated and restarted according to its restart policy.
Other management of the container blocks until the hook completes.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                preStop
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PreStop is called immediately before a container is terminated due to an
API request or management event such as liveness/startup probe failure,
preemption, resource contention, etc. The handler is not called if the
container crashes or exits. The Pod's termination grace period countdown begins before the
PreStop hook is executed. Regardless of the outcome of the handler, the
container will eventually terminate within the Pod's termination grace
period (unless delayed by finalizers). Other management of the container blocks until the hook completes
or until the termination grace period is reached.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                stopSignal
                   | String
                   | doc m%"
StopSignal defines which signal will be sent to a container when it is being stopped.
If not specified, the default is defined by the container runtime in use.
StopSignal can only be set for Pods with a non-empty .spec.os.name
"%
                   | optional
              }
                 | doc "Lifecycle is not allowed for ephemeral containers."
                 | optional,
              livenessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc "Probes are not allowed for ephemeral containers."
                 | optional,
              name
                 | String
                 | doc m%"
Name of the ephemeral container specified as a DNS_LABEL.
This name must be unique among all containers, init containers and ephemeral containers.
"%,
              ports
                 | Array {
                containerPort
                   | Number
                   | doc m%"
Number of port to expose on the pod's IP address.
This must be a valid port number, 0 < x < 65536.
"%,
                hostIP
                   | String
                   | doc "What host IP to bind the external port to."
                   | optional,
                hostPort
                   | Number
                   | doc m%"
Number of port to expose on the host.
If specified, this must be a valid port number, 0 < x < 65536.
If HostNetwork is specified, this must match ContainerPort.
Most containers do not need this.
"%
                   | optional,
                name
                   | String
                   | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each
named port in a pod must have a unique name. Name for the port that can be
referred to by services.
"%
                   | optional,
                protocol
                   | String
                   | doc m%"
Protocol for port. Must be UDP, TCP, or SCTP.
Defaults to "TCP".
"%
                   = "TCP"
              }
                 | doc "Ports are not allowed for ephemeral containers."
                 | optional,
              readinessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc "Probes are not allowed for ephemeral containers."
                 | optional,
              resizePolicy
                 | Array {
                resourceName
                   | String
                   | doc m%"
Name of the resource to which this resource resize policy applies.
Supported values: cpu, memory.
"%,
                restartPolicy
                   | String
                   | doc m%"
Restart policy to apply when specified resource is resized.
If not specified, it defaults to NotRequired.
"%
              }
                 | doc "Resources resize policy for the container."
                 | optional,
              resources
                 | {
                claims
                   | Array {
                  name
                     | String
                     | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of
the Pod where this field is used. It makes that resource available
inside a container.
"%,
                  request
                     | String
                     | doc m%"
Request is the name chosen for a request in the referenced claim.
If empty, everything from the claim is made available, otherwise
only the result of this request.
"%
                     | optional
                }
                   | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims,
that are used by this container.
This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.
This field is immutable. It can only be set for containers.
"%
                   | optional,
                limits
                   | {
                }
                   | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional,
                requests
                   | {
                }
                   | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional
              }
                 | doc m%"
Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources
already allocated to the pod.
"%
                 | optional,
              restartPolicy
                 | String
                 | doc m%"
Restart policy for the container to manage the restart behavior of each
container within a pod.
This may only be set for init containers. You cannot set this field on
ephemeral containers.
"%
                 | optional,
              securityContext
                 | {
                allowPrivilegeEscalation
                   | Bool
                   | doc m%"
AllowPrivilegeEscalation controls whether a process can gain more
privileges than its parent process. This bool directly controls if
the no_new_privs flag will be set on the container process.
AllowPrivilegeEscalation is true always when the container is:
1) run as Privileged
2) has CAP_SYS_ADMIN
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                appArmorProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%
                }
                   | doc m%"
appArmorProfile is the AppArmor options to use by this container. If set, this profile
overrides the pod's appArmorProfile.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                capabilities
                   | {
                  add
                     | Array String
                     | doc "Added capabilities"
                     | optional,
                  drop
                     | Array String
                     | doc "Removed capabilities"
                     | optional
                }
                   | doc m%"
The capabilities to add/drop when running containers.
Defaults to the default set of capabilities granted by the container runtime.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                privileged
                   | Bool
                   | doc m%"
Run container in privileged mode.
Processes in privileged containers are essentially equivalent to root on the host.
Defaults to false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                procMount
                   | String
                   | doc m%"
procMount denotes the type of proc mount to use for the containers.
The default value is Default which uses the container runtime defaults for
readonly paths and masked paths.
This requires the ProcMountType feature flag to be enabled.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                readOnlyRootFilesystem
                   | Bool
                   | doc m%"
Whether this container has a read-only root filesystem.
Default is false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsGroup
                   | Number
                   | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsNonRoot
                   | Bool
                   | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                   | optional,
                runAsUser
                   | Number
                   | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seLinuxOptions
                   | {
                  level
                     | String
                     | doc "Level is SELinux level label that applies to the container."
                     | optional,
                  role
                     | String
                     | doc "Role is a SELinux role label that applies to the container."
                     | optional,
                  type
                     | String
                     | doc "Type is a SELinux type label that applies to the container."
                     | optional,
                  user
                     | String
                     | doc "User is a SELinux user label that applies to the container."
                     | optional
                }
                   | doc m%"
The SELinux context to be applied to the container.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seccompProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:
Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%
                }
                   | doc m%"
The seccomp options to use by this container. If seccomp options are
provided at both the pod & container level, the container options
override the pod options.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                windowsOptions
                   | {
                  gmsaCredentialSpec
                     | String
                     | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%
                     | optional,
                  gmsaCredentialSpecName
                     | String
                     | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use."
                     | optional,
                  hostProcess
                     | Bool
                     | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%
                     | optional,
                  runAsUserName
                     | String
                     | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                     | optional
                }
                   | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options from the PodSecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%
                   | optional
              }
                 | doc m%"
Optional: SecurityContext defines the security options the ephemeral container should be run with.
If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
"%
                 | optional,
              startupProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc "Probes are not allowed for ephemeral containers."
                 | optional,
              stdin
                 | Bool
                 | doc m%"
Whether this container should allocate a buffer for stdin in the container runtime. If this
is not set, reads from stdin in the container will always result in EOF.
Default is false.
"%
                 | optional,
              stdinOnce
                 | Bool
                 | doc m%"
Whether the container runtime should close the stdin channel after it has been opened by
a single attach. When stdin is true the stdin stream will remain open across multiple attach
sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the
first client attaches to stdin, and then remains open and accepts data until the client disconnects,
at which time stdin is closed and remains closed until the container is restarted. If this
flag is false, a container processes that reads from stdin will never receive an EOF.
Default is false
"%
                 | optional,
              targetContainerName
                 | String
                 | doc m%"
If set, the name of the container from PodSpec that this ephemeral container targets.
The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container.
If not set then the ephemeral container uses the namespaces configured in the Pod spec.
The container runtime must implement support for this feature. If the runtime does not
support namespace targeting then the result of setting this field is undefined.
"%
                 | optional,
              terminationMessagePath
                 | String
                 | doc m%"
Optional: Path at which the file to which the container's termination message
will be written is mounted into the container's filesystem.
Message written is intended to be brief final status, such as an assertion failure message.
Will be truncated by the node if greater than 4096 bytes. The total message length across
all containers will be limited to 12kb.
Defaults to /dev/termination-log.
Cannot be updated.
"%
                 | optional,
              terminationMessagePolicy
                 | String
                 | doc m%"
Indicate how the termination message should be populated. File will use the contents of
terminationMessagePath to populate the container status message on both success and failure.
FallbackToLogsOnError will use the last chunk of container log output if the termination
message file is empty and the container exited with an error.
The log output is limited to 2048 bytes or 80 lines, whichever is smaller.
Defaults to File.
Cannot be updated.
"%
                 | optional,
              tty
                 | Bool
                 | doc m%"
Whether this container should allocate a TTY for itself, also requires 'stdin' to be true.
Default is false.
"%
                 | optional,
              volumeDevices
                 | Array {
                devicePath
                   | String
                   | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
                name
                   | String
                   | doc "name must match the name of a persistentVolumeClaim in the pod"
              }
                 | doc "volumeDevices is the list of block devices to be used by the container."
                 | optional,
              volumeMounts
                 | Array k8s_io_api_core_v1_VolumeMount.VolumeMount
                 | doc m%"
Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers.
Cannot be updated.
"%
                 | optional,
              workingDir
                 | String
                 | doc m%"
Container's working directory.
If not specified, the container runtime's default will be used, which
might be configured in the container image.
Cannot be updated.
"%
                 | optional
            }
               | doc m%"
List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing
pod to perform user-initiated actions such as debugging. This list cannot be specified when
creating a pod, and it cannot be modified by updating the pod spec. In order to add an
ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
"%
               | optional,
            hostAliases
               | Array {
              hostnames
                 | Array String
                 | doc "Hostnames for the above IP address."
                 | optional,
              ip
                 | String
                 | doc "IP address of the host file entry."
            }
               | doc m%"
HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts
file if specified.
"%
               | optional,
            hostIPC
               | Bool
               | doc m%"
Use the host's ipc namespace.
Optional: Default to false.
"%
               | optional,
            hostNetwork
               | Bool
               | doc m%"
Host networking requested for this pod. Use the host's network namespace.
If this option is set, the ports that will be used must be specified.
Default to false.
"%
               | optional,
            hostPID
               | Bool
               | doc m%"
Use the host's pid namespace.
Optional: Default to false.
"%
               | optional,
            hostUsers
               | Bool
               | doc m%"
Use the host's user namespace.
Optional: Default to true.
If set to true or not present, the pod will be run in the host user namespace, useful
for when the pod needs a feature only available to the host user namespace, such as
loading a kernel module with CAP_SYS_MODULE.
When set to false, a new userns is created for the pod. Setting false is useful for
mitigating container breakout vulnerabilities even allowing users to run their
containers as root without actually having root privileges on the host.
This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
"%
               | optional,
            hostname
               | String
               | doc m%"
Specifies the hostname of the Pod
If not specified, the pod's hostname will be set to a system-defined value.
"%
               | optional,
            imagePullSecrets
               | Array {
              name
                 | String
                 | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                 = ""
            }
               | doc m%"
ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
If specified, these secrets will be passed to individual puller implementations for them to use.
More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
"%
               | optional,
            initContainers
               | Array {
              args
                 | Array String
                 | doc m%"
Arguments to the entrypoint.
The container image's CMD is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              command
                 | Array String
                 | doc m%"
Entrypoint array. Not executed within a shell.
The container image's ENTRYPOINT is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced
to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will
produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless
of whether the variable exists or not. Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
                 | optional,
              env
                 | Array k8s_io_api_core_v1_EnvVar.EnvVar
                 | doc m%"
List of environment variables to set in the container.
Cannot be updated.
"%
                 | optional,
              envFrom
                 | Array k8s_io_api_core_v1_EnvFromSource.EnvFromSource
                 | doc m%"
List of sources to populate environment variables in the container.
The keys defined within a source must be a C_IDENTIFIER. All invalid keys
will be reported as an event when the container is starting. When a key exists in multiple
sources, the value associated with the last source will take precedence.
Values defined by an Env with a duplicate key will take precedence.
Cannot be updated.
"%
                 | optional,
              image
                 | String
                 | doc m%"
Container image name.
More info: https://kubernetes.io/docs/concepts/containers/images
This field is optional to allow higher level config management to default or override
container images in workload controllers like Deployments and StatefulSets.
"%
                 | optional,
              imagePullPolicy
                 | String
                 | doc m%"
Image pull policy.
One of Always, Never, IfNotPresent.
Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%
                 | optional,
              lifecycle
                 | {
                postStart
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PostStart is called immediately after a container is created. If the handler fails,
the container is terminated and restarted according to its restart policy.
Other management of the container blocks until the hook completes.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                preStop
                   | {
                  exec
                     | {
                    command
                       | Array String
                       | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                       | optional
                  }
                     | doc "Exec specifies a command to execute in the container."
                     | optional,
                  httpGet
                     | {
                    host
                       | String
                       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                       | optional,
                    httpHeaders
                       | Array {
                      name
                         | String
                         | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                      value
                         | String
                         | doc "The header field value"
                    }
                       | doc "Custom headers to set in the request. HTTP allows repeated headers."
                       | optional,
                    path
                       | String
                       | doc "Path to access on the HTTP server."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                    scheme
                       | String
                       | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                       | optional
                  }
                     | doc "HTTPGet specifies an HTTP GET request to perform."
                     | optional,
                  sleep
                     | {
                    seconds
                       | Number
                       | doc "Seconds is the number of seconds to sleep."
                  }
                     | doc "Sleep represents a duration that the container should sleep."
                     | optional,
                  tcpSocket
                     | {
                    host
                       | String
                       | doc "Optional: Host name to connect to, defaults to the pod IP."
                       | optional,
                    port
                       | Number | String
                       | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                  }
                     | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept
for backward compatibility. There is no validation of this field and
lifecycle hooks will fail at runtime when it is specified.
"%
                     | optional
                }
                   | doc m%"
PreStop is called immediately before a container is terminated due to an
API request or management event such as liveness/startup probe failure,
preemption, resource contention, etc. The handler is not called if the
container crashes or exits. The Pod's termination grace period countdown begins before the
PreStop hook is executed. Regardless of the outcome of the handler, the
container will eventually terminate within the Pod's termination grace
period (unless delayed by finalizers). Other management of the container blocks until the hook completes
or until the termination grace period is reached.
More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
                   | optional,
                stopSignal
                   | String
                   | doc m%"
StopSignal defines which signal will be sent to a container when it is being stopped.
If not specified, the default is defined by the container runtime in use.
StopSignal can only be set for Pods with a non-empty .spec.os.name
"%
                   | optional
              }
                 | doc m%"
Actions that the management system should take in response to container lifecycle events.
Cannot be updated.
"%
                 | optional,
              livenessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
Periodic probe of container liveness.
Container will be restarted if the probe fails.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              name
                 | String
                 | doc m%"
Name of the container specified as a DNS_LABEL.
Each container in a pod must have a unique name (DNS_LABEL).
Cannot be updated.
"%,
              ports
                 | Array {
                containerPort
                   | Number
                   | doc m%"
Number of port to expose on the pod's IP address.
This must be a valid port number, 0 < x < 65536.
"%,
                hostIP
                   | String
                   | doc "What host IP to bind the external port to."
                   | optional,
                hostPort
                   | Number
                   | doc m%"
Number of port to expose on the host.
If specified, this must be a valid port number, 0 < x < 65536.
If HostNetwork is specified, this must match ContainerPort.
Most containers do not need this.
"%
                   | optional,
                name
                   | String
                   | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each
named port in a pod must have a unique name. Name for the port that can be
referred to by services.
"%
                   | optional,
                protocol
                   | String
                   | doc m%"
Protocol for port. Must be UDP, TCP, or SCTP.
Defaults to "TCP".
"%
                   = "TCP"
              }
                 | doc m%"
List of ports to expose from the container. Not specifying a port here
DOES NOT prevent that port from being exposed. Any port which is
listening on the default "0.0.0.0" address inside a container will be
accessible from the network.
Modifying this array with strategic merge patch may corrupt the data.
For more information See https://github.com/kubernetes/kubernetes/issues/108255.
Cannot be updated.
"%
                 | optional,
              readinessProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
Periodic probe of container service readiness.
Container will be removed from service endpoints if the probe fails.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              resizePolicy
                 | Array {
                resourceName
                   | String
                   | doc m%"
Name of the resource to which this resource resize policy applies.
Supported values: cpu, memory.
"%,
                restartPolicy
                   | String
                   | doc m%"
Restart policy to apply when specified resource is resized.
If not specified, it defaults to NotRequired.
"%
              }
                 | doc "Resources resize policy for the container."
                 | optional,
              resources
                 | {
                claims
                   | Array {
                  name
                     | String
                     | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of
the Pod where this field is used. It makes that resource available
inside a container.
"%,
                  request
                     | String
                     | doc m%"
Request is the name chosen for a request in the referenced claim.
If empty, everything from the claim is made available, otherwise
only the result of this request.
"%
                     | optional
                }
                   | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims,
that are used by this container.
This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.
This field is immutable. It can only be set for containers.
"%
                   | optional,
                limits
                   | {
                }
                   | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional,
                requests
                   | {
                }
                   | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                   | optional
              }
                 | doc m%"
Compute Resources required by this container.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                 | optional,
              restartPolicy
                 | String
                 | doc m%"
RestartPolicy defines the restart behavior of individual containers in a pod.
This field may only be set for init containers, and the only allowed value is "Always".
For non-init containers or when this field is not specified,
the restart behavior is defined by the Pod's restart policy and the container type.
Setting the RestartPolicy as "Always" for the init container will have the following effect:
this init container will be continually restarted on
exit until all regular containers have terminated. Once all regular
containers have completed, all init containers with restartPolicy "Always"
will be shut down. This lifecycle differs from normal init containers and
is often referred to as a "sidecar" container. Although this init
container still starts in the init container sequence, it does not wait
for the container to complete before proceeding to the next init
container. Instead, the next init container starts immediately after this
init container is started, or after any startupProbe has successfully
completed.
"%
                 | optional,
              securityContext
                 | {
                allowPrivilegeEscalation
                   | Bool
                   | doc m%"
AllowPrivilegeEscalation controls whether a process can gain more
privileges than its parent process. This bool directly controls if
the no_new_privs flag will be set on the container process.
AllowPrivilegeEscalation is true always when the container is:
1) run as Privileged
2) has CAP_SYS_ADMIN
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                appArmorProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%
                }
                   | doc m%"
appArmorProfile is the AppArmor options to use by this container. If set, this profile
overrides the pod's appArmorProfile.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                capabilities
                   | {
                  add
                     | Array String
                     | doc "Added capabilities"
                     | optional,
                  drop
                     | Array String
                     | doc "Removed capabilities"
                     | optional
                }
                   | doc m%"
The capabilities to add/drop when running containers.
Defaults to the default set of capabilities granted by the container runtime.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                privileged
                   | Bool
                   | doc m%"
Run container in privileged mode.
Processes in privileged containers are essentially equivalent to root on the host.
Defaults to false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                procMount
                   | String
                   | doc m%"
procMount denotes the type of proc mount to use for the containers.
The default value is Default which uses the container runtime defaults for
readonly paths and masked paths.
This requires the ProcMountType feature flag to be enabled.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                readOnlyRootFilesystem
                   | Bool
                   | doc m%"
Whether this container has a read-only root filesystem.
Default is false.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsGroup
                   | Number
                   | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                runAsNonRoot
                   | Bool
                   | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                   | optional,
                runAsUser
                   | Number
                   | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seLinuxOptions
                   | {
                  level
                     | String
                     | doc "Level is SELinux level label that applies to the container."
                     | optional,
                  role
                     | String
                     | doc "Role is a SELinux role label that applies to the container."
                     | optional,
                  type
                     | String
                     | doc "Type is a SELinux type label that applies to the container."
                     | optional,
                  user
                     | String
                     | doc "User is a SELinux user label that applies to the container."
                     | optional
                }
                   | doc m%"
The SELinux context to be applied to the container.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                seccompProfile
                   | {
                  localhostProfile
                     | String
                     | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%
                     | optional,
                  type
                     | String
                     | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:
Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%
                }
                   | doc m%"
The seccomp options to use by this container. If seccomp options are
provided at both the pod & container level, the container options
override the pod options.
Note that this field cannot be set when spec.os.name is windows.
"%
                   | optional,
                windowsOptions
                   | {
                  gmsaCredentialSpec
                     | String
                     | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%
                     | optional,
                  gmsaCredentialSpecName
                     | String
                     | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use."
                     | optional,
                  hostProcess
                     | Bool
                     | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%
                     | optional,
                  runAsUserName
                     | String
                     | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                     | optional
                }
                   | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options from the PodSecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%
                   | optional
              }
                 | doc m%"
SecurityContext defines the security options the container should be run with.
If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
"%
                 | optional,
              startupProbe
                 | {
                exec
                   | {
                  command
                     | Array String
                     | doc m%"
Command is the command line to execute inside the container, the working directory for the
command  is root ('/') in the container's filesystem. The command is simply exec'd, it is
not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use
a shell, you need to explicitly call out to that shell.
Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
                     | optional
                }
                   | doc "Exec specifies a command to execute in the container."
                   | optional,
                failureThreshold
                   | Number
                   | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded.
Defaults to 3. Minimum value is 1.
"%
                   | optional,
                grpc
                   | {
                  port
                     | Number
                     | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
                  service
                     | String
                     | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest
(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
If this is not specified, the default behavior is defined by gRPC.
"%
                     = ""
                }
                   | doc "GRPC specifies a GRPC HealthCheckRequest."
                   | optional,
                httpGet
                   | {
                  host
                     | String
                     | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set
"Host" in httpHeaders instead.
"%
                     | optional,
                  httpHeaders
                     | Array {
                    name
                       | String
                       | doc m%"
The header field name.
This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
                    value
                       | String
                       | doc "The header field value"
                  }
                     | doc "Custom headers to set in the request. HTTP allows repeated headers."
                     | optional,
                  path
                     | String
                     | doc "Path to access on the HTTP server."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Name or number of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%,
                  scheme
                     | String
                     | doc m%"
Scheme to use for connecting to the host.
Defaults to HTTP.
"%
                     | optional
                }
                   | doc "HTTPGet specifies an HTTP GET request to perform."
                   | optional,
                initialDelaySeconds
                   | Number
                   | doc m%"
Number of seconds after the container has started before liveness probes are initiated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional,
                periodSeconds
                   | Number
                   | doc m%"
How often (in seconds) to perform the probe.
Default to 10 seconds. Minimum value is 1.
"%
                   | optional,
                successThreshold
                   | Number
                   | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed.
Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
                   | optional,
                tcpSocket
                   | {
                  host
                     | String
                     | doc "Optional: Host name to connect to, defaults to the pod IP."
                     | optional,
                  port
                     | Number | String
                     | doc m%"
Number or name of the port to access on the container.
Number must be in the range 1 to 65535.
Name must be an IANA_SVC_NAME.
"%
                }
                   | doc "TCPSocket specifies a connection to a TCP port."
                   | optional,
                terminationGracePeriodSeconds
                   | Number
                   | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this
value overrides the value provided by the pod spec.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate.
Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
                   | optional,
                timeoutSeconds
                   | Number
                   | doc m%"
Number of seconds after which the probe times out.
Defaults to 1 second. Minimum value is 1.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                   | optional
              }
                 | doc m%"
StartupProbe indicates that the Pod has successfully initialized.
If specified, no other probes are executed until this completes successfully.
If this probe fails, the Pod will be restarted, just as if the livenessProbe failed.
This can be used to provide different probe parameters at the beginning of a Pod's lifecycle,
when it might take a long time to load data or warm a cache, than during steady-state operation.
This cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
                 | optional,
              stdin
                 | Bool
                 | doc m%"
Whether this container should allocate a buffer for stdin in the container runtime. If this
is not set, reads from stdin in the container will always result in EOF.
Default is false.
"%
                 | optional,
              stdinOnce
                 | Bool
                 | doc m%"
Whether the container runtime should close the stdin channel after it has been opened by
a single attach. When stdin is true the stdin stream will remain open across multiple attach
sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the
first client attaches to stdin, and then remains open and accepts data until the client disconnects,
at which time stdin is closed and remains closed until the container is restarted. If this
flag is false, a container processes that reads from stdin will never receive an EOF.
Default is false
"%
                 | optional,
              terminationMessagePath
                 | String
                 | doc m%"
Optional: Path at which the file to which the container's termination message
will be written is mounted into the container's filesystem.
Message written is intended to be brief final status, such as an assertion failure message.
Will be truncated by the node if greater than 4096 bytes. The total message length across
all containers will be limited to 12kb.
Defaults to /dev/termination-log.
Cannot be updated.
"%
                 | optional,
              terminationMessagePolicy
                 | String
                 | doc m%"
Indicate how the termination message should be populated. File will use the contents of
terminationMessagePath to populate the container status message on both success and failure.
FallbackToLogsOnError will use the last chunk of container log output if the termination
message file is empty and the container exited with an error.
The log output is limited to 2048 bytes or 80 lines, whichever is smaller.
Defaults to File.
Cannot be updated.
"%
                 | optional,
              tty
                 | Bool
                 | doc m%"
Whether this container should allocate a TTY for itself, also requires 'stdin' to be true.
Default is false.
"%
                 | optional,
              volumeDevices
                 | Array {
                devicePath
                   | String
                   | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
                name
                   | String
                   | doc "name must match the name of a persistentVolumeClaim in the pod"
              }
                 | doc "volumeDevices is the list of block devices to be used by the container."
                 | optional,
              volumeMounts
                 | Array k8s_io_api_core_v1_VolumeMount.VolumeMount
                 | doc m%"
Pod volumes to mount into the container's filesystem.
Cannot be updated.
"%
                 | optional,
              workingDir
                 | String
                 | doc m%"
Container's working directory.
If not specified, the container runtime's default will be used, which
might be configured in the container image.
Cannot be updated.
"%
                 | optional
            }
               | doc m%"
List of initialization containers belonging to the pod.
Init containers are executed in order prior to containers being started. If any
init container fails, the pod is considered to have failed and is handled according
to its restartPolicy. The name for an init container or normal container must be
unique among all containers.
Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes.
The resourceRequirements of an init container are taken into account during scheduling
by finding the highest request/limit for each resource type, and then using the max of
that value or the sum of the normal containers. Limits are applied to init containers
in a similar fashion.
Init containers cannot currently be added or removed.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
"%
               | optional,
            nodeName
               | String
               | doc m%"
NodeName indicates in which node this pod is scheduled.
If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName.
Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod.
This field should not be used to express a desire for the pod to be scheduled on a specific node.
https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename
"%
               | optional,
            nodeSelector
               | {
            }
               | doc m%"
NodeSelector is a selector which must be true for the pod to fit on a node.
Selector which must match a node's labels for the pod to be scheduled on that node.
More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
"%
               | optional,
            os
               | {
              name
                 | String
                 | doc m%"
Name is the name of the operating system. The currently supported values are linux and windows.
Additional value may be defined in future and can be one of:
https://github.com/opencontainers/runtime-spec/blob/master/config.md#platform-specific-configuration
Clients should expect to handle additional values and treat unrecognized values in this field as os: null
"%
            }
               | doc m%"
Specifies the OS of the containers in the pod.
Some pod and container fields are restricted if this is set.
If the OS field is set to linux, the following fields must be unset:
-securityContext.windowsOptions
If the OS field is set to windows, following fields must be unset:
- spec.hostPID
- spec.hostIPC
- spec.hostUsers
- spec.securityContext.appArmorProfile
- spec.securityContext.seLinuxOptions
- spec.securityContext.seccompProfile
- spec.securityContext.fsGroup
- spec.securityContext.fsGroupChangePolicy
- spec.securityContext.sysctls
- spec.shareProcessNamespace
- spec.securityContext.runAsUser
- spec.securityContext.runAsGroup
- spec.securityContext.supplementalGroups
- spec.securityContext.supplementalGroupsPolicy
- spec.containers[*].securityContext.appArmorProfile
- spec.containers[*].securityContext.seLinuxOptions
- spec.containers[*].securityContext.seccompProfile
- spec.containers[*].securityContext.capabilities
- spec.containers[*].securityContext.readOnlyRootFilesystem
- spec.containers[*].securityContext.privileged
- spec.containers[*].securityContext.allowPrivilegeEscalation
- spec.containers[*].securityContext.procMount
- spec.containers[*].securityContext.runAsUser
- spec.containers[*].securityContext.runAsGroup
"%
               | optional,
            overhead
               | {
            }
               | doc m%"
Overhead represents the resource overhead associated with running a pod for a given RuntimeClass.
This field will be autopopulated at admission time by the RuntimeClass admission controller. If
the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests.
The RuntimeClass admission controller will reject Pod create requests which have the overhead already
set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value
defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero.
More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
"%
               | optional,
            preemptionPolicy
               | String
               | doc m%"
PreemptionPolicy is the Policy for preempting pods with lower priority.
One of Never, PreemptLowerPriority.
Defaults to PreemptLowerPriority if unset.
"%
               | optional,
            "priority"
               | Number
               | doc m%"
The priority value. Various system components use this field to find the
priority of the pod. When Priority Admission Controller is enabled, it
prevents users from setting this field. The admission controller populates
this field from PriorityClassName.
The higher the value, the higher the priority.
"%
               | optional,
            priorityClassName
               | String
               | doc m%"
If specified, indicates the pod's priority. "system-node-critical" and
"system-cluster-critical" are two special keywords which indicate the
highest priorities with the former being the highest priority. Any other
name must be defined by creating a PriorityClass object with that name.
If not specified, the pod priority will be default or zero if there is no
default.
"%
               | optional,
            readinessGates
               | Array {
              conditionType
                 | String
                 | doc m%"
ConditionType refers to a condition in the pod's condition list with matching type.
"%
            }
               | doc m%"
If specified, all readiness gates will be evaluated for pod readiness.
A pod is ready when all its containers are ready AND
all conditions specified in the readiness gates have status equal to "True"
More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
"%
               | optional,
            resourceClaims
               | Array {
              name
                 | String
                 | doc m%"
Name uniquely identifies this resource claim inside the pod.
This must be a DNS_LABEL.
"%,
              resourceClaimName
                 | String
                 | doc m%"
ResourceClaimName is the name of a ResourceClaim object in the same
namespace as this pod.
Exactly one of ResourceClaimName and ResourceClaimTemplateName must
be set.
"%
                 | optional,
              resourceClaimTemplateName
                 | String
                 | doc m%"
ResourceClaimTemplateName is the name of a ResourceClaimTemplate
object in the same namespace as this pod.
The template will be used to create a new ResourceClaim, which will
be bound to this pod. When this pod is deleted, the ResourceClaim
will also be deleted. The pod name and resource name, along with a
generated component, will be used to form a unique name for the
ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.
This field is immutable and no changes will be made to the
corresponding ResourceClaim by the control plane after creating the
ResourceClaim.
Exactly one of ResourceClaimName and ResourceClaimTemplateName must
be set.
"%
                 | optional
            }
               | doc m%"
ResourceClaims defines which ResourceClaims must be allocated
and reserved before the Pod is allowed to start. The resources
will be made available to those containers which consume them
by name.
This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.
This field is immutable.
"%
               | optional,
            resources
               | {
              claims
                 | Array {
                name
                   | String
                   | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of
the Pod where this field is used. It makes that resource available
inside a container.
"%,
                request
                   | String
                   | doc m%"
Request is the name chosen for a request in the referenced claim.
If empty, everything from the claim is made available, otherwise
only the result of this request.
"%
                   | optional
              }
                 | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims,
that are used by this container.
This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.
This field is immutable. It can only be set for containers.
"%
                 | optional,
              limits
                 | {
              }
                 | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                 | optional,
              requests
                 | {
              }
                 | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                 | optional
            }
               | doc m%"
Resources is the total amount of CPU and Memory resources required by all
containers in the pod. It supports specifying Requests and Limits for
"cpu" and "memory" resource names only. ResourceClaims are not supported.
This field enables fine-grained control over resource allocation for the
entire pod, allowing resource sharing among containers in a pod.
This is an alpha field and requires enabling the PodLevelResources feature
gate.
"%
               | optional,
            restartPolicy
               | String
               | doc m%"
Restart policy for all containers within the pod.
One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted.
Default to Always.
More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
"%
               | optional,
            runtimeClassName
               | String
               | doc m%"
RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used
to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run.
If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an
empty definition that uses the default runtime handler.
More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
"%
               | optional,
            schedulerName
               | String
               | doc m%"
If specified, the pod will be dispatched by specified scheduler.
If not specified, the pod will be dispatched by default scheduler.
"%
               | optional,
            schedulingGates
               | Array {
              name
                 | String
                 | doc m%"
Name of the scheduling gate.
Each scheduling gate must have a unique name field.
"%
            }
               | doc m%"
SchedulingGates is an opaque list of values that if specified will block scheduling the pod.
If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the
scheduler will not attempt to schedule the pod.
SchedulingGates can only be set at pod creation time, and be removed only afterwards.
"%
               | optional,
            securityContext
               | {
              appArmorProfile
                 | {
                localhostProfile
                   | String
                   | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%
                   | optional,
                type
                   | String
                   | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%
              }
                 | doc m%"
appArmorProfile is the AppArmor options to use by the containers in this pod.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              fsGroup
                 | Number
                 | doc m%"
A special supplemental group that applies to all containers in a pod.
Some volume types allow the Kubelet to change the ownership of that volume
to be owned by the pod:
1. The owning GID will be the FSGroup
2. The setgid bit is set (new files created in the volume will be owned by FSGroup)
3. The permission bits are OR'd with rw-rw----
If unset, the Kubelet will not modify the ownership and permissions of any volume.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              fsGroupChangePolicy
                 | String
                 | doc m%"
fsGroupChangePolicy defines behavior of changing ownership and permission of the volume
before being exposed inside Pod. This field will only apply to
volume types which support fsGroup based ownership(and permissions).
It will have no effect on ephemeral volume types such as: secret, configmaps
and emptydir.
Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              runAsGroup
                 | Number
                 | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence
for that container.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              runAsNonRoot
                 | Bool
                 | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                 | optional,
              runAsUser
                 | Number
                 | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence
for that container.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              seLinuxChangePolicy
                 | String
                 | doc m%"
seLinuxChangePolicy defines how the container's SELinux label is applied to all volumes used by the Pod.
It has no effect on nodes that do not support SELinux or to volumes does not support SELinux.
Valid values are "MountOption" and "Recursive".
"Recursive" means relabeling of all files on all Pod volumes by the container runtime.
This may be slow for large volumes, but allows mixing privileged and unprivileged Pods sharing the same volume on the same node.
"MountOption" mounts all eligible Pod volumes with `-o context` mount option.
This requires all Pods that share the same volume to use the same SELinux label.
It is not possible to share the same volume among privileged and unprivileged Pods.
Eligible volumes are in-tree FibreChannel and iSCSI volumes, and all CSI volumes
whose CSI driver announces SELinux support by setting spec.seLinuxMount: true in their
CSIDriver instance. Other volumes are always re-labelled recursively.
"MountOption" value is allowed only when SELinuxMount feature gate is enabled.
If not specified and SELinuxMount feature gate is enabled, "MountOption" is used.
If not specified and SELinuxMount feature gate is disabled, "MountOption" is used for ReadWriteOncePod volumes
and "Recursive" for all other volumes.
This field affects only Pods that have SELinux label set, either in PodSecurityContext or in SecurityContext of all containers.
All Pods that use the same volume should use the same seLinuxChangePolicy, otherwise some pods can get stuck in ContainerCreating state.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              seLinuxOptions
                 | {
                level
                   | String
                   | doc "Level is SELinux level label that applies to the container."
                   | optional,
                role
                   | String
                   | doc "Role is a SELinux role label that applies to the container."
                   | optional,
                type
                   | String
                   | doc "Type is a SELinux type label that applies to the container."
                   | optional,
                user
                   | String
                   | doc "User is a SELinux user label that applies to the container."
                   | optional
              }
                 | doc m%"
The SELinux context to be applied to all containers.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in SecurityContext.  If set in
both SecurityContext and PodSecurityContext, the value specified in SecurityContext
takes precedence for that container.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              seccompProfile
                 | {
                localhostProfile
                   | String
                   | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%
                   | optional,
                type
                   | String
                   | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:
Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%
              }
                 | doc m%"
The seccomp options to use by the containers in this pod.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              supplementalGroups
                 | Array Number
                 | doc m%"
A list of groups applied to the first process run in each container, in
addition to the container's primary GID and fsGroup (if specified).  If
the SupplementalGroupsPolicy feature is enabled, the
supplementalGroupsPolicy field determines whether these are in addition
to or instead of any group memberships defined in the container image.
If unspecified, no additional groups are added, though group memberships
defined in the container image may still be used, depending on the
supplementalGroupsPolicy field.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              supplementalGroupsPolicy
                 | String
                 | doc m%"
Defines how supplemental groups of the first container processes are calculated.
Valid values are "Merge" and "Strict". If not specified, "Merge" is used.
(Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled
and the container runtime must implement support for this feature.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              sysctls
                 | Array {
                name
                   | String
                   | doc "Name of a property to set",
                value
                   | String
                   | doc "Value of a property to set"
              }
                 | doc m%"
Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported
sysctls (by the container runtime) might fail to launch.
Note that this field cannot be set when spec.os.name is windows.
"%
                 | optional,
              windowsOptions
                 | {
                gmsaCredentialSpec
                   | String
                   | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%
                   | optional,
                gmsaCredentialSpecName
                   | String
                   | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use."
                   | optional,
                hostProcess
                   | Bool
                   | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%
                   | optional,
                runAsUserName
                   | String
                   | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
                   | optional
              }
                 | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options within a container's SecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%
                 | optional
            }
               | doc m%"
SecurityContext holds pod-level security attributes and common container settings.
Optional: Defaults to empty.  See type description for default values of each field.
"%
               | optional,
            serviceAccount
               | String
               | doc m%"
DeprecatedServiceAccount is a deprecated alias for ServiceAccountName.
Deprecated: Use serviceAccountName instead.
"%
               | optional,
            serviceAccountName
               | String
               | doc m%"
ServiceAccountName is the name of the ServiceAccount to use to run this pod.
More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
"%
               | optional,
            setHostnameAsFQDN
               | Bool
               | doc m%"
If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default).
In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname).
In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN.
If a pod does not have FQDN, this has no effect.
Default to false.
"%
               | optional,
            shareProcessNamespace
               | Bool
               | doc m%"
Share a single process namespace between all of the containers in a pod.
When this is set containers will be able to view and signal processes from other containers
in the same pod, and the first process in each container will not be assigned PID 1.
HostPID and ShareProcessNamespace cannot both be set.
Optional: Default to false.
"%
               | optional,
            subdomain
               | String
               | doc m%"
If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>".
If not specified, the pod will not have a domainname at all.
"%
               | optional,
            terminationGracePeriodSeconds
               | Number
               | doc m%"
Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request.
Value must be non-negative integer. The value zero indicates stop immediately via
the kill signal (no opportunity to shut down).
If this value is nil, the default grace period will be used instead.
The grace period is the duration in seconds after the processes running in the pod are sent
a termination signal and the time when the processes are forcibly halted with a kill signal.
Set this value longer than the expected cleanup time for your process.
Defaults to 30 seconds.
"%
               | optional,
            tolerations
               | Array k8s_io_api_core_v1_Toleration.Toleration
               | doc "If specified, the pod's tolerations."
               | optional,
            topologySpreadConstraints
               | Array {
              labelSelector
                 | {
                matchExpressions
                   | Array {
                  key
                     | String
                     | doc "key is the label key that the selector applies to.",
                  operator
                     | String
                     | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values
                     | Array String
                     | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                     | optional
                }
                   | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                   | optional,
                matchLabels
                   | {
                }
                   | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                   | optional
              }
                 | doc m%"
LabelSelector is used to find matching pods.
Pods that match this label selector are counted to determine the number of pods
in their corresponding topology domain.
"%
                 | optional,
              matchLabelKeys
                 | Array String
                 | doc m%"
MatchLabelKeys is a set of pod label keys to select the pods over which
spreading will be calculated. The keys are used to lookup values from the
incoming pod labels, those key-value labels are ANDed with labelSelector
to select the group of existing pods over which spreading will be calculated
for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector.
MatchLabelKeys cannot be set when LabelSelector isn't set.
Keys that don't exist in the incoming pod labels will
be ignored. A null or empty list means only match against labelSelector.
This is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).
"%
                 | optional,
              maxSkew
                 | Number
                 | doc m%"
MaxSkew describes the degree to which pods may be unevenly distributed.
When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference
between the number of matching pods in the target topology and the global minimum.
The global minimum is the minimum number of matching pods in an eligible domain
or zero if the number of eligible domains is less than MinDomains.
For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
labelSelector spread as 2/2/1:
In this case, the global minimum is 1.
| zone1 | zone2 | zone3 |
|  P P  |  P P  |   P   |
- if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2;
scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2)
violate MaxSkew(1).
- if MaxSkew is 2, incoming pod can be scheduled onto any zone.
When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence
to topologies that satisfy it.
It's a required field. Default value is 1 and 0 is not allowed.
"%,
              minDomains
                 | Number
                 | doc m%"
MinDomains indicates a minimum number of eligible domains.
When the number of eligible domains with matching topology keys is less than minDomains,
Pod Topology Spread treats "global minimum" as 0, and then the calculation of Skew is performed.
And when the number of eligible domains with matching topology keys equals or greater than minDomains,
this value has no effect on scheduling.
As a result, when the number of eligible domains is less than minDomains,
scheduler won't schedule more than maxSkew Pods to those domains.
If value is nil, the constraint behaves as if MinDomains is equal to 1.
Valid values are integers greater than 0.
When value is not nil, WhenUnsatisfiable must be DoNotSchedule.
For example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same
labelSelector spread as 2/2/2:
| zone1 | zone2 | zone3 |
|  P P  |  P P  |  P P  |
The number of domains is less than 5(MinDomains), so "global minimum" is treated as 0.
In this situation, new pod with the same labelSelector cannot be scheduled,
because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones,
it will violate MaxSkew.
"%
                 | optional,
              nodeAffinityPolicy
                 | String
                 | doc m%"
NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector
when calculating pod topology spread skew. Options are:
- Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations.
- Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.
If this value is nil, the behavior is equivalent to the Honor policy.
"%
                 | optional,
              nodeTaintsPolicy
                 | String
                 | doc m%"
NodeTaintsPolicy indicates how we will treat node taints when calculating
pod topology spread skew. Options are:
- Honor: nodes without taints, along with tainted nodes for which the incoming pod
has a toleration, are included.
- Ignore: node taints are ignored. All nodes are included.
If this value is nil, the behavior is equivalent to the Ignore policy.
"%
                 | optional,
              topologyKey
                 | String
                 | doc m%"
TopologyKey is the key of node labels. Nodes that have a label with this key
and identical values are considered to be in the same topology.
We consider each <key, value> as a "bucket", and try to put balanced number
of pods into each bucket.
We define a domain as a particular instance of a topology.
Also, we define an eligible domain as a domain whose nodes meet the requirements of
nodeAffinityPolicy and nodeTaintsPolicy.
e.g. If TopologyKey is "kubernetes.io/hostname", each Node is a domain of that topology.
And, if TopologyKey is "topology.kubernetes.io/zone", each zone is a domain of that topology.
It's a required field.
"%,
              whenUnsatisfiable
                 | String
                 | doc m%"
WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy
the spread constraint.
- DoNotSchedule (default) tells the scheduler not to schedule it.
- ScheduleAnyway tells the scheduler to schedule the pod in any location,
  but giving higher precedence to topologies that would help reduce the
  skew.
A constraint is considered "Unsatisfiable" for an incoming pod
if and only if every possible node assignment for that pod would violate
"MaxSkew" on some topology.
For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
labelSelector spread as 3/1/1:
| zone1 | zone2 | zone3 |
| P P P |   P   |   P   |
If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled
to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies
MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler
won't make it *more* imbalanced.
It's a required field.
"%
            }
               | doc m%"
TopologySpreadConstraints describes how a group of pods ought to spread across topology
domains. Scheduler will schedule pods in a way which abides by the constraints.
All topologySpreadConstraints are ANDed.
"%
               | optional,
            volumes
               | Array {
              awsElasticBlockStore
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
                   | optional,
                partition
                   | Number
                   | doc m%"
partition is the partition in the volume that you want to mount.
If omitted, the default is to mount by volume name.
Examples: For volume /dev/sda1, you specify the partition as "1".
Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly value true will force the readOnly setting in VolumeMounts.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
                   | optional,
                volumeID
                   | String
                   | doc m%"
volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume).
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
              }
                 | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree
awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
                 | optional,
              azureDisk
                 | {
                cachingMode
                   | String
                   | doc "cachingMode is the Host Caching mode: None, Read Only, Read Write."
                   | optional,
                diskName
                   | String
                   | doc "diskName is the Name of the data disk in the blob storage",
                diskURI
                   | String
                   | doc "diskURI is the URI of data disk in the blob storage",
                fsType
                   | String
                   | doc m%"
fsType is Filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   = "ext4",
                kind
                   | String
                   | doc m%"
kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   = false
              }
                 | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type
are redirected to the disk.csi.azure.com CSI driver.
"%
                 | optional,
              azureFile
                 | {
                readOnly
                   | Bool
                   | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                secretName
                   | String
                   | doc m%"
secretName is the  name of secret that contains Azure Storage Account Name and Key
"%,
                shareName
                   | String
                   | doc "shareName is the azure share Name"
              }
                 | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod.
Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type
are redirected to the file.csi.azure.com CSI driver.
"%
                 | optional,
              cephfs
                 | {
                monitors
                   | Array String
                   | doc m%"
monitors is Required: Monitors is a collection of Ceph monitors
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
                path
                   | String
                   | doc m%"
path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
                   | optional,
                secretFile
                   | String
                   | doc m%"
secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty.
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
                   | optional,
                user
                   | String
                   | doc m%"
user is optional: User is the rados user name, default is admin
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
                   | optional
              }
                 | doc m%"
cephFS represents a Ceph FS mount on the host that shares a pod's lifetime.
Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
"%
                 | optional,
              cinder
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef is optional: points to a secret object containing parameters used to connect
to OpenStack.
"%
                   | optional,
                volumeID
                   | String
                   | doc m%"
volumeID used to identify the volume in cinder.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
              }
                 | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine.
Deprecated: Cinder is deprecated. All operations for the in-tree cinder type
are redirected to the cinder.csi.openstack.org CSI driver.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
                 | optional,
              configMap
                 | {
                defaultMode
                   | Number
                   | doc m%"
defaultMode is optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                   | optional,
                items
                   | Array {
                  key
                     | String
                     | doc "key is the key to project.",
                  mode
                     | Number
                     | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                     | optional,
                  path
                     | String
                     | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%
                }
                   | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
ConfigMap will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the ConfigMap,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%
                   | optional,
                name
                   | String
                   | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                   = "",
                "optional"
                   | Bool
                   | doc "optional specify whether the ConfigMap or its keys must be defined"
                   | optional
              }
                 | doc "configMap represents a configMap that should populate this volume"
                 | optional,
              csi
                 | {
                driver
                   | String
                   | doc m%"
driver is the name of the CSI driver that handles this volume.
Consult with your admin for the correct name as registered in the cluster.
"%,
                fsType
                   | String
                   | doc m%"
fsType to mount. Ex. "ext4", "xfs", "ntfs".
If not provided, the empty value is passed to the associated CSI driver
which will determine the default filesystem to apply.
"%
                   | optional,
                nodePublishSecretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
nodePublishSecretRef is a reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
NodePublishVolume and NodeUnpublishVolume calls.
This field is optional, and  may be empty if no secret is required. If the
secret object contains more than one secret, all secret references are passed.
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly specifies a read-only configuration for the volume.
Defaults to false (read/write).
"%
                   | optional,
                volumeAttributes
                   | {
                }
                   | doc m%"
volumeAttributes stores driver-specific properties that are passed to the CSI
driver. Consult your driver's documentation for supported values.
"%
                   | optional
              }
                 | doc m%"
csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers.
"%
                 | optional,
              downwardAPI
                 | {
                defaultMode
                   | Number
                   | doc m%"
Optional: mode bits to use on created files by default. Must be a
Optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                   | optional,
                items
                   | Array {
                  fieldRef
                     | {
                    apiVersion
                       | String
                       | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\"."
                       | optional,
                    fieldPath
                       | String
                       | doc "Path of the field to select in the specified API version."
                  }
                     | doc m%"
Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
"%
                     | optional,
                  mode
                     | Number
                     | doc m%"
Optional: mode bits used to set permissions on this file, must be an octal value
between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                     | optional,
                  path
                     | String
                     | doc m%"
Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
"%,
                  resourceFieldRef
                     | {
                    containerName
                       | String
                       | doc "Container name: required for volumes, optional for env vars"
                       | optional,
                    divisor
                       | Number | String
                       | doc "Specifies the output format of the exposed resources, defaults to \"1\""
                       | optional,
                    resource
                       | String
                       | doc "Required: resource to select"
                  }
                     | doc m%"
Selects a resource of the container: only resources limits and requests
(limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
"%
                     | optional
                }
                   | doc "Items is a list of downward API volume file"
                   | optional
              }
                 | doc m%"
downwardAPI represents downward API about the pod that should populate this volume
"%
                 | optional,
              emptyDir
                 | {
                medium
                   | String
                   | doc m%"
medium represents what type of storage medium should back this directory.
The default is "" which means to use the node's default medium.
Must be an empty string (default) or Memory.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
                   | optional,
                sizeLimit
                   | Number | String
                   | doc m%"
sizeLimit is the total amount of local storage required for this EmptyDir volume.
The size limit is also applicable for memory medium.
The maximum usage on memory medium EmptyDir would be the minimum value between
the SizeLimit specified here and the sum of memory limits of all containers in a pod.
The default is nil which means that the limit is undefined.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
                   | optional
              }
                 | doc m%"
emptyDir represents a temporary directory that shares a pod's lifetime.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
                 | optional,
              ephemeral
                 | {
                volumeClaimTemplate
                   | {
                  metadata
                     | {
                    annotations
                       | {
                    }
                       | optional,
                    finalizers
                       | Array String
                       | optional,
                    labels
                       | {
                    }
                       | optional,
                    name
                       | String
                       | optional,
                    namespace
                       | String
                       | optional
                  }
                     | doc m%"
May contain labels and annotations that will be copied into the PVC
when creating it. No other fields are allowed and will be rejected during
validation.
"%
                     | optional,
                  spec
                     | {
                    accessModes
                       | Array String
                       | doc m%"
accessModes contains the desired access modes the volume should have.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
"%
                       | optional,
                    dataSource
                       | {
                      apiGroup
                         | String
                         | doc m%"
APIGroup is the group for the resource being referenced.
If APIGroup is not specified, the specified Kind must be in the core API group.
For any other third-party types, APIGroup is required.
"%
                         | optional,
                      kind
                         | String
                         | doc "Kind is the type of resource being referenced",
                      name
                         | String
                         | doc "Name is the name of resource being referenced"
                    }
                       | doc m%"
dataSource field can be used to specify either:
* An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot)
* An existing PVC (PersistentVolumeClaim)
If the provisioner or an external controller can support the specified data source,
it will create a new volume based on the contents of the specified data source.
When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef,
and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified.
If the namespace is specified, then dataSourceRef will not be copied to dataSource.
"%
                       | optional,
                    dataSourceRef
                       | {
                      apiGroup
                         | String
                         | doc m%"
APIGroup is the group for the resource being referenced.
If APIGroup is not specified, the specified Kind must be in the core API group.
For any other third-party types, APIGroup is required.
"%
                         | optional,
                      kind
                         | String
                         | doc "Kind is the type of resource being referenced",
                      name
                         | String
                         | doc "Name is the name of resource being referenced",
                      namespace
                         | String
                         | doc m%"
Namespace is the namespace of resource being referenced
Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details.
(Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%
                         | optional
                    }
                       | doc m%"
dataSourceRef specifies the object from which to populate the volume with data, if a non-empty
volume is desired. This may be any object from a non-empty API group (non
core object) or a PersistentVolumeClaim object.
When this field is specified, volume binding will only succeed if the type of
the specified object matches some installed volume populator or dynamic
provisioner.
This field will replace the functionality of the dataSource field and as such
if both fields are non-empty, they must have the same value. For backwards
compatibility, when namespace isn't specified in dataSourceRef,
both fields (dataSource and dataSourceRef) will be set to the same
value automatically if one of them is empty and the other is non-empty.
When namespace is specified in dataSourceRef,
dataSource isn't set to the same value and must be empty.
There are three important differences between dataSource and dataSourceRef:
* While dataSource only allows two specific types of objects, dataSourceRef
  allows any non-core object, as well as PersistentVolumeClaim objects.
* While dataSource ignores disallowed values (dropping them), dataSourceRef
  preserves all values, and generates an error if a disallowed value is
  specified.
* While dataSource only allows local objects, dataSourceRef allows objects
  in any namespaces.
(Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
(Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%
                       | optional,
                    resources
                       | {
                      limits
                         | {
                      }
                         | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                         | optional,
                      requests
                         | {
                      }
                         | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
                         | optional
                    }
                       | doc m%"
resources represents the minimum resources the volume should have.
If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements
that are lower than previous value but must still be higher than capacity recorded in the
status field of the claim.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
"%
                       | optional,
                    selector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc "selector is a label query over volumes to consider for binding."
                       | optional,
                    storageClassName
                       | String
                       | doc m%"
storageClassName is the name of the StorageClass required by the claim.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
"%
                       | optional,
                    volumeAttributesClassName
                       | String
                       | doc m%"
volumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim.
If specified, the CSI driver will create or update the volume with the attributes defined
in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName,
it can be changed after the claim is created. An empty string value means that no VolumeAttributesClass
will be applied to the claim but it's not allowed to reset this field to empty string once it is set.
If unspecified and the PersistentVolumeClaim is unbound, the default VolumeAttributesClass
will be set by the persistentvolume controller if it exists.
If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be
set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource
exists.
More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/
(Beta) Using this field requires the VolumeAttributesClass feature gate to be enabled (off by default).
"%
                       | optional,
                    volumeMode
                       | String
                       | doc m%"
volumeMode defines what type of volume is required by the claim.
Value of Filesystem is implied when not included in claim spec.
"%
                       | optional,
                    volumeName
                       | String
                       | doc "volumeName is the binding reference to the PersistentVolume backing this claim."
                       | optional
                  }
                     | doc m%"
The specification for the PersistentVolumeClaim. The entire content is
copied unchanged into the PVC that gets created from this
template. The same fields as in a PersistentVolumeClaim
are also valid here.
"%
                }
                   | doc m%"
Will be used to create a stand-alone PVC to provision the volume.
The pod in which this EphemeralVolumeSource is embedded will be the
owner of the PVC, i.e. the PVC will be deleted together with the
pod.  The name of the PVC will be `<pod name>-<volume name>` where
`<volume name>` is the name from the `PodSpec.Volumes` array
entry. Pod validation will reject the pod if the concatenated name
is not valid for a PVC (for example, too long).
An existing PVC with that name that is not owned by the pod
will *not* be used for the pod to avoid using an unrelated
volume by mistake. Starting the pod is then blocked until
the unrelated PVC is removed. If such a pre-created PVC is
meant to be used by the pod, the PVC has to updated with an
owner reference to the pod once the pod exists. Normally
this should not be necessary, but it may be useful when
manually reconstructing a broken cluster.
This field is read-only and no changes will be made by Kubernetes
to the PVC after it has been created.
Required, must not be nil.
"%
                   | optional
              }
                 | doc m%"
ephemeral represents a volume that is handled by a cluster storage driver.
The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts,
and deleted when the pod is removed.
Use this if:
a) the volume is only needed while the pod runs,
b) features of normal volumes like restoring from snapshot or capacity
   tracking are needed,
c) the storage driver is specified through a storage class, and
d) the storage driver supports dynamic volume provisioning through
   a PersistentVolumeClaim (see EphemeralVolumeSource for more
   information on the connection between this volume type
   and PersistentVolumeClaim).
Use PersistentVolumeClaim or one of the vendor-specific
APIs for volumes that persist for longer than the lifecycle
of an individual pod.
Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to
be used that way - see the documentation of the driver for
more information.
A pod can use both types of ephemeral volumes and
persistent volumes at the same time.
"%
                 | optional,
              fc
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   | optional,
                lun
                   | Number
                   | doc "lun is Optional: FC target lun number"
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                targetWWNs
                   | Array String
                   | doc "targetWWNs is Optional: FC target worldwide names (WWNs)"
                   | optional,
                wwids
                   | Array String
                   | doc m%"
wwids Optional: FC volume world wide identifiers (wwids)
Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
"%
                   | optional
              }
                 | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%
                 | optional,
              flexVolume
                 | {
                driver
                   | String
                   | doc "driver is the name of the driver to use for this volume.",
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%
                   | optional,
                options
                   | {
                }
                   | doc "options is Optional: this field holds extra command options if any."
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef is Optional: secretRef is reference to the secret object containing
sensitive information to pass to the plugin scripts. This may be
empty if no secret object is specified. If the secret object
contains more than one secret, all secrets are passed to the plugin
scripts.
"%
                   | optional
              }
                 | doc m%"
flexVolume represents a generic volume resource that is
provisioned/attached using an exec based plugin.
Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
"%
                 | optional,
              flocker
                 | {
                datasetName
                   | String
                   | doc m%"
datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker
should be considered as deprecated
"%
                   | optional,
                datasetUUID
                   | String
                   | doc m%"
datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
"%
                   | optional
              }
                 | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running.
Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
"%
                 | optional,
              gcePersistentDisk
                 | {
                fsType
                   | String
                   | doc m%"
fsType is filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
                   | optional,
                partition
                   | Number
                   | doc m%"
partition is the partition in the volume that you want to mount.
If omitted, the default is to mount by volume name.
Examples: For volume /dev/sda1, you specify the partition as "1".
Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
                   | optional,
                pdName
                   | String
                   | doc m%"
pdName is unique name of the PD resource in GCE. Used to identify the disk in GCE.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
                   | optional
              }
                 | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree
gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
                 | optional,
              gitRepo
                 | {
                directory
                   | String
                   | doc m%"
directory is the target directory name.
Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the
git repository.  Otherwise, if specified, the volume will contain the git repository in
the subdirectory with the given name.
"%
                   | optional,
                repository
                   | String
                   | doc "repository is the URL",
                revision
                   | String
                   | doc "revision is the commit hash for the specified revision."
                   | optional
              }
                 | doc m%"
gitRepo represents a git repository at a particular revision.
Deprecated: GitRepo is deprecated. To provision a container with a git repo, mount an
EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir
into the Pod's container.
"%
                 | optional,
              glusterfs
                 | {
                endpoints
                   | String
                   | doc m%"
endpoints is the endpoint name that details Glusterfs topology.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
                path
                   | String
                   | doc m%"
path is the Glusterfs volume path.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the Glusterfs volume to be mounted with read-only permissions.
Defaults to false.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%
                   | optional
              }
                 | doc m%"
glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime.
Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported.
More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%
                 | optional,
              hostPath
                 | {
                path
                   | String
                   | doc m%"
path of the directory on the host.
If the path is a symlink, it will follow the link to the real path.
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
                type
                   | String
                   | doc m%"
type for HostPath Volume
Defaults to ""
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
                   | optional
              }
                 | doc m%"
hostPath represents a pre-existing file or directory on the host
machine that is directly exposed to the container. This is generally
used for system agents or other privileged things that are allowed
to see the host machine. Most containers will NOT need this.
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
                 | optional,
              image
                 | {
                pullPolicy
                   | String
                   | doc m%"
Policy for pulling OCI objects. Possible values are:
Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails.
Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present.
IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.
Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
"%
                   | optional,
                reference
                   | String
                   | doc m%"
Required: Image or artifact reference to be used.
Behaves in the same way as pod.spec.containers[*].image.
Pull secrets will be assembled in the same way as for the container image by looking up node credentials, SA image pull secrets, and pod spec image pull secrets.
More info: https://kubernetes.io/docs/concepts/containers/images
This field is optional to allow higher level config management to default or override
container images in workload controllers like Deployments and StatefulSets.
"%
                   | optional
              }
                 | doc m%"
image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine.
The volume is resolved at pod startup depending on which PullPolicy value is provided:
- Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails.
- Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present.
- IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.
The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation.
A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message.
The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field.
The OCI object gets mounted in a single directory (spec.containers[*].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images.
The volume will be mounted read-only (ro) and non-executable files (noexec).
Sub path mounts for containers are not supported (spec.containers[*].volumeMounts.subpath) before 1.33.
The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
"%
                 | optional,
              iscsi
                 | {
                chapAuthDiscovery
                   | Bool
                   | doc "chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication"
                   | optional,
                chapAuthSession
                   | Bool
                   | doc "chapAuthSession defines whether support iSCSI Session CHAP authentication"
                   | optional,
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
"%
                   | optional,
                initiatorName
                   | String
                   | doc m%"
initiatorName is the custom iSCSI Initiator Name.
If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface
<target portal>:<volume name> will be created for the connection.
"%
                   | optional,
                iqn
                   | String
                   | doc "iqn is the target iSCSI Qualified Name.",
                iscsiInterface
                   | String
                   | doc m%"
iscsiInterface is the interface Name that uses an iSCSI transport.
Defaults to 'default' (tcp).
"%
                   = "default",
                lun
                   | Number
                   | doc "lun represents iSCSI Target Lun number.",
                portals
                   | Array String
                   | doc m%"
portals is the iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port
is other than default (typically TCP ports 860 and 3260).
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc "secretRef is the CHAP Secret for iSCSI target and initiator authentication"
                   | optional,
                targetPortal
                   | String
                   | doc m%"
targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port
is other than default (typically TCP ports 860 and 3260).
"%
              }
                 | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
More info: https://examples.k8s.io/volumes/iscsi/README.md
"%
                 | optional,
              name
                 | String
                 | doc m%"
name of the volume.
Must be a DNS_LABEL and unique within the pod.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%,
              nfs
                 | {
                path
                   | String
                   | doc m%"
path that is exported by the NFS server.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the NFS export to be mounted with read-only permissions.
Defaults to false.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
                   | optional,
                server
                   | String
                   | doc m%"
server is the hostname or IP address of the NFS server.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
              }
                 | doc m%"
nfs represents an NFS mount on the host that shares a pod's lifetime
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
                 | optional,
              persistentVolumeClaim
                 | {
                claimName
                   | String
                   | doc m%"
claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%,
                readOnly
                   | Bool
                   | doc m%"
readOnly Will force the ReadOnly setting in VolumeMounts.
Default false.
"%
                   | optional
              }
                 | doc m%"
persistentVolumeClaimVolumeSource represents a reference to a
PersistentVolumeClaim in the same namespace.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
                 | optional,
              photonPersistentDisk
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   | optional,
                pdID
                   | String
                   | doc "pdID is the ID that identifies Photon Controller persistent disk"
              }
                 | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine.
Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
"%
                 | optional,
              portworxVolume
                 | {
                fsType
                   | String
                   | doc m%"
fSType represents the filesystem type to mount
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                volumeID
                   | String
                   | doc "volumeID uniquely identifies a Portworx volume"
              }
                 | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine.
Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type
are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate
is on.
"%
                 | optional,
              projected
                 | {
                defaultMode
                   | Number
                   | doc m%"
defaultMode are the mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                   | optional,
                sources
                   | Array {
                  clusterTrustBundle
                     | {
                    labelSelector
                       | {
                      matchExpressions
                         | Array {
                        key
                           | String
                           | doc "key is the label key that the selector applies to.",
                        operator
                           | String
                           | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                        values
                           | Array String
                           | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%
                           | optional
                      }
                         | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%
                         | optional,
                      matchLabels
                         | {
                      }
                         | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%
                         | optional
                    }
                       | doc m%"
Select all ClusterTrustBundles that match this label selector.  Only has
effect if signerName is set.  Mutually-exclusive with name.  If unset,
interpreted as "match nothing".  If set but empty, interpreted as "match
everything".
"%
                       | optional,
                    name
                       | String
                       | doc m%"
Select a single ClusterTrustBundle by object name.  Mutually-exclusive
with signerName and labelSelector.
"%
                       | optional,
                    "optional"
                       | Bool
                       | doc m%"
If true, don't block pod startup if the referenced ClusterTrustBundle(s)
aren't available.  If using name, then the named ClusterTrustBundle is
allowed not to exist.  If using signerName, then the combination of
signerName and labelSelector is allowed to match zero
ClusterTrustBundles.
"%
                       | optional,
                    path
                       | String
                       | doc "Relative path from the volume root to write the bundle.",
                    signerName
                       | String
                       | doc m%"
Select all ClusterTrustBundles that match this signer name.
Mutually-exclusive with name.  The contents of all selected
ClusterTrustBundles will be unified and deduplicated.
"%
                       | optional
                  }
                     | doc m%"
ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field
of ClusterTrustBundle objects in an auto-updating file.
Alpha, gated by the ClusterTrustBundleProjection feature gate.
ClusterTrustBundle objects can either be selected by name, or by the
combination of signer name and a label selector.
Kubelet performs aggressive normalization of the PEM contents written
into the pod filesystem.  Esoteric PEM features such as inter-block
comments and block headers are stripped.  Certificates are deduplicated.
The ordering of certificates within the file is arbitrary, and Kubelet
may change the order over time.
"%
                     | optional,
                  configMap
                     | {
                    items
                       | Array {
                      key
                         | String
                         | doc "key is the key to project.",
                      mode
                         | Number
                         | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                         | optional,
                      path
                         | String
                         | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%
                    }
                       | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
ConfigMap will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the ConfigMap,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%
                       | optional,
                    name
                       | String
                       | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                       = "",
                    "optional"
                       | Bool
                       | doc "optional specify whether the ConfigMap or its keys must be defined"
                       | optional
                  }
                     | doc "configMap information about the configMap data to project"
                     | optional,
                  downwardAPI
                     | {
                    items
                       | Array {
                      fieldRef
                         | {
                        apiVersion
                           | String
                           | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\"."
                           | optional,
                        fieldPath
                           | String
                           | doc "Path of the field to select in the specified API version."
                      }
                         | doc m%"
Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
"%
                         | optional,
                      mode
                         | Number
                         | doc m%"
Optional: mode bits used to set permissions on this file, must be an octal value
between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                         | optional,
                      path
                         | String
                         | doc m%"
Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
"%,
                      resourceFieldRef
                         | {
                        containerName
                           | String
                           | doc "Container name: required for volumes, optional for env vars"
                           | optional,
                        divisor
                           | Number | String
                           | doc "Specifies the output format of the exposed resources, defaults to \"1\""
                           | optional,
                        resource
                           | String
                           | doc "Required: resource to select"
                      }
                         | doc m%"
Selects a resource of the container: only resources limits and requests
(limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
"%
                         | optional
                    }
                       | doc "Items is a list of DownwardAPIVolume file"
                       | optional
                  }
                     | doc "downwardAPI information about the downwardAPI data to project"
                     | optional,
                  secret
                     | {
                    items
                       | Array {
                      key
                         | String
                         | doc "key is the key to project.",
                      mode
                         | Number
                         | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                         | optional,
                      path
                         | String
                         | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%
                    }
                       | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
Secret will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the Secret,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%
                       | optional,
                    name
                       | String
                       | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                       = "",
                    "optional"
                       | Bool
                       | doc "optional field specify whether the Secret or its key must be defined"
                       | optional
                  }
                     | doc "secret information about the secret data to project"
                     | optional,
                  serviceAccountToken
                     | {
                    audience
                       | String
                       | doc m%"
audience is the intended audience of the token. A recipient of a token
must identify itself with an identifier specified in the audience of the
token, and otherwise should reject the token. The audience defaults to the
identifier of the apiserver.
"%
                       | optional,
                    expirationSeconds
                       | Number
                       | doc m%"
expirationSeconds is the requested duration of validity of the service
account token. As the token approaches expiration, the kubelet volume
plugin will proactively rotate the service account token. The kubelet will
start trying to rotate the token if the token is older than 80 percent of
its time to live or if the token is older than 24 hours.Defaults to 1 hour
and must be at least 10 minutes.
"%
                       | optional,
                    path
                       | String
                       | doc m%"
path is the path relative to the mount point of the file to project the
token into.
"%
                  }
                     | doc "serviceAccountToken is information about the serviceAccountToken data to project"
                     | optional
                }
                   | doc m%"
sources is the list of volume projections. Each entry in this list
handles one source.
"%
                   | optional
              }
                 | doc "projected items for all in one resources secrets, configmaps, and downward API"
                 | optional,
              quobyte
                 | {
                group
                   | String
                   | doc m%"
group to map volume access to
Default is no group
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the Quobyte volume to be mounted with read-only permissions.
Defaults to false.
"%
                   | optional,
                registry
                   | String
                   | doc m%"
registry represents a single or multiple Quobyte Registry services
specified as a string as host:port pair (multiple entries are separated with commas)
which acts as the central registry for volumes
"%,
                tenant
                   | String
                   | doc m%"
tenant owning the given Quobyte volume in the Backend
Used with dynamically provisioned Quobyte volumes, value is set by the plugin
"%
                   | optional,
                user
                   | String
                   | doc m%"
user to map volume access to
Defaults to serivceaccount user
"%
                   | optional,
                volume
                   | String
                   | doc "volume is a string that references an already created Quobyte volume by name."
              }
                 | doc m%"
quobyte represents a Quobyte mount on the host that shares a pod's lifetime.
Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
"%
                 | optional,
              rbd
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
"%
                   | optional,
                image
                   | String
                   | doc m%"
image is the rados image name.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
                keyring
                   | String
                   | doc m%"
keyring is the path to key ring for RBDUser.
Default is /etc/ceph/keyring.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
                   = "/etc/ceph/keyring",
                monitors
                   | Array String
                   | doc m%"
monitors is a collection of Ceph monitors.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
                pool
                   | String
                   | doc m%"
pool is the rados pool name.
Default is rbd.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
                   = "rbd",
                readOnly
                   | Bool
                   | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef is name of the authentication secret for RBDUser. If provided
overrides keyring.
Default is nil.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
                   | optional,
                user
                   | String
                   | doc m%"
user is the rados user name.
Default is admin.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
                   = "admin"
              }
                 | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime.
Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported.
More info: https://examples.k8s.io/volumes/rbd/README.md
"%
                 | optional,
              scaleIO
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs".
Default is "xfs".
"%
                   = "xfs",
                gateway
                   | String
                   | doc "gateway is the host address of the ScaleIO API Gateway.",
                protectionDomain
                   | String
                   | doc m%"
protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef references to the secret for ScaleIO user and other
sensitive information. If this is not provided, Login operation will fail.
"%,
                sslEnabled
                   | Bool
                   | doc "sslEnabled Flag enable/disable SSL communication with Gateway, default false"
                   | optional,
                storageMode
                   | String
                   | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned.
Default is ThinProvisioned.
"%
                   = "ThinProvisioned",
                storagePool
                   | String
                   | doc "storagePool is the ScaleIO Storage Pool associated with the protection domain."
                   | optional,
                system
                   | String
                   | doc "system is the name of the storage system as configured in ScaleIO.",
                volumeName
                   | String
                   | doc m%"
volumeName is the name of a volume already created in the ScaleIO system
that is associated with this volume source.
"%
                   | optional
              }
                 | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
"%
                 | optional,
              secret
                 | {
                defaultMode
                   | Number
                   | doc m%"
defaultMode is Optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values
for mode bits. Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                   | optional,
                items
                   | Array {
                  key
                     | String
                     | doc "key is the key to project.",
                  mode
                     | Number
                     | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%
                     | optional,
                  path
                     | String
                     | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%
                }
                   | doc m%"
items If unspecified, each key-value pair in the Data field of the referenced
Secret will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the Secret,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%
                   | optional,
                "optional"
                   | Bool
                   | doc "optional field specify whether the Secret or its keys must be defined"
                   | optional,
                secretName
                   | String
                   | doc m%"
secretName is the name of the secret in the pod's namespace to use.
More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%
                   | optional
              }
                 | doc m%"
secret represents a secret that should populate this volume.
More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%
                 | optional,
              storageos
                 | {
                fsType
                   | String
                   | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   | optional,
                readOnly
                   | Bool
                   | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%
                   | optional,
                secretRef
                   | {
                  name
                     | String
                     | doc m%"
Name of the referent.
This field is effectively required, but due to backwards compatibility is
allowed to be empty. Instances of this type with an empty value here are
almost certainly wrong.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
                     = ""
                }
                   | doc m%"
secretRef specifies the secret to use for obtaining the StorageOS API
credentials.  If not specified, default values will be attempted.
"%
                   | optional,
                volumeName
                   | String
                   | doc m%"
volumeName is the human-readable name of the StorageOS volume.  Volume
names are only unique within a namespace.
"%
                   | optional,
                volumeNamespace
                   | String
                   | doc m%"
volumeNamespace specifies the scope of the volume within StorageOS.  If no
namespace is specified then the Pod's namespace will be used.  This allows the
Kubernetes name scoping to be mirrored within StorageOS for tighter integration.
Set VolumeName to any name to override the default behaviour.
Set to "default" if you are not using namespaces within StorageOS.
Namespaces that do not pre-exist within StorageOS will be created.
"%
                   | optional
              }
                 | doc m%"
storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported.
"%
                 | optional,
              vsphereVolume
                 | {
                fsType
                   | String
                   | doc m%"
fsType is filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
                   | optional,
                storagePolicyID
                   | String
                   | doc m%"
storagePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
"%
                   | optional,
                storagePolicyName
                   | String
                   | doc "storagePolicyName is the storage Policy Based Management (SPBM) profile name."
                   | optional,
                volumePath
                   | String
                   | doc "volumePath is the path that identifies vSphere volume vmdk"
              }
                 | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine.
Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type
are redirected to the csi.vsphere.vmware.com CSI driver.
"%
                 | optional
            }
               | doc m%"
List of volumes that can be mounted by containers belonging to the pod.
More info: https://kubernetes.io/docs/concepts/storage/volumes
"%
               | optional
          }
             | doc m%"
Specification of the desired behavior of the pod.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
             | optional
        }
           | doc m%"
Template describes the pods that will be created.
The only allowed template.spec.restartPolicy value is "Always".
"%
      }
         | doc "Spec contains the configurable spec fields for the Deployment object."
         | optional
    }
       | doc "DeploymentTemplate is the template for the Deployment object."
       | optional,
    serviceAccountTemplate
       | {
      metadata
         | {
        annotations
           | {
        }
           | doc m%"
Annotations is an unstructured key value map stored with a resource that
may be set by external tools to store and retrieve arbitrary metadata.
They are not queryable and should be preserved when modifying objects.
More info: http:https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/
"%
           | optional,
        labels
           | {
        }
           | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. Labels will be merged with internal labels
used by crossplane, and labels with a crossplane.io key might be
overwritten.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%
           | optional,
        name
           | String
           | doc "Name is the name of the object."
           | optional
      }
         | doc "Metadata contains the configurable metadata fields for the ServiceAccount."
         | optional
    }
       | doc "ServiceAccountTemplate is the template for the ServiceAccount object."
       | optional,
    serviceTemplate
       | {
      metadata
         | {
        annotations
           | {
        }
           | doc m%"
Annotations is an unstructured key value map stored with a resource that
may be set by external tools to store and retrieve arbitrary metadata.
They are not queryable and should be preserved when modifying objects.
More info: http:https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/
"%
           | optional,
        labels
           | {
        }
           | doc m%"
Map of string keys and values that can be used to organize and categorize
(scope and select) objects. Labels will be merged with internal labels
used by crossplane, and labels with a crossplane.io key might be
overwritten.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%
           | optional,
        name
           | String
           | doc "Name is the name of the object."
           | optional
      }
         | doc "Metadata contains the configurable metadata fields for the Service."
         | optional
    }
       | doc "ServiceTemplate is the template for the Service object."
       | optional
  }
     | doc m%"
DeploymentRuntimeConfigSpec specifies the configuration for a packaged controller.
Values provided will override package manager defaults. Labels and
annotations are passed to both the controller Deployment and ServiceAccount.
"%
     | optional
}