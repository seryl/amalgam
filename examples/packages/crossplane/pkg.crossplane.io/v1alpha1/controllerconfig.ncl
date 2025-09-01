# Module: controllerconfig.pkg.crossplane.io

let k8s_io_objectmeta = import "k8s_io" in
let k8s_io_toleration = import "k8s_io" in
let k8s_io_envfromsource = import "k8s_io" in
let k8s_io_volumemount = import "k8s_io" in
let k8s_io_envvar = import "k8s_io" in

{
  ControllerConfig = {
    apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object.
Servers should convert recognized schemas to the latest internal value, and
may reject unrecognized values.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
    kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents.
Servers may infer this from the endpoint the client submits requests to.
Cannot be updated.
In CamelCase.
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
    metadata | optional | k8s_io_objectmeta.ObjectMeta,
    spec | optional | {
      affinity | optional | {
        nodeAffinity | optional | {
          preferredDuringSchedulingIgnoredDuringExecution | optional | Array {
            preference | {
              matchExpressions | optional | Array {
                key | String | doc "The label key that the selector applies to.",
                operator | String | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%,
              } | doc "A list of node selector requirements by node's labels.",
              matchFields | optional | Array {
                key | String | doc "The label key that the selector applies to.",
                operator | String | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%,
              } | doc "A list of node selector requirements by node's fields.",
            } | doc "A node selector term, associated with the corresponding weight.",
            weight | Number | doc m%"
Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
"%,
          } | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node matches the corresponding matchExpressions; the
node(s) with the highest sum are the most preferred.
"%,
          requiredDuringSchedulingIgnoredDuringExecution | optional | {
            nodeSelectorTerms | Array {
              matchExpressions | optional | Array {
                key | String | doc "The label key that the selector applies to.",
                operator | String | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%,
              } | doc "A list of node selector requirements by node's labels.",
              matchFields | optional | Array {
                key | String | doc "The label key that the selector applies to.",
                operator | String | doc m%"
Represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
                values | optional | Array String | doc m%"
An array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. If the operator is Gt or Lt, the values
array must have a single element, which will be interpreted as an integer.
This array is replaced during a strategic merge patch.
"%,
              } | doc "A list of node selector requirements by node's fields.",
            } | doc "Required. A list of node selector terms. The terms are ORed.",
          } | doc m%"
If the affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to an update), the system
may or may not try to eventually evict the pod from its node.
"%,
        } | doc "Describes node affinity scheduling rules for the pod.",
        podAffinity | optional | {
          preferredDuringSchedulingIgnoredDuringExecution | optional | Array {
            podAffinityTerm | {
              labelSelector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%,
              matchLabelKeys | optional | Array String | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
              mismatchLabelKeys | optional | Array String | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
              namespaceSelector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%,
              namespaces | optional | Array String | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%,
              topologyKey | String | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%,
            } | doc "Required. A pod affinity term, associated with the corresponding weight.",
            weight | Number | doc m%"
weight associated with matching the corresponding podAffinityTerm,
in the range 1-100.
"%,
          } | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
node(s) with the highest sum are the most preferred.
"%,
          requiredDuringSchedulingIgnoredDuringExecution | optional | Array {
            labelSelector | optional | {
              matchExpressions | optional | Array {
                key | String | doc "key is the label key that the selector applies to.",
                operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
              } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
              matchLabels | optional | {
              } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
            } | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%,
            matchLabelKeys | optional | Array String | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
            mismatchLabelKeys | optional | Array String | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
            namespaceSelector | optional | {
              matchExpressions | optional | Array {
                key | String | doc "key is the label key that the selector applies to.",
                operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
              } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
              matchLabels | optional | {
              } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
            } | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%,
            namespaces | optional | Array String | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%,
            topologyKey | String | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%,
          } | doc m%"
If the affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to a pod label update), the
system may or may not try to eventually evict the pod from its node.
When there are multiple elements, the lists of nodes corresponding to each
podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%,
        } | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%,
        podAntiAffinity | optional | {
          preferredDuringSchedulingIgnoredDuringExecution | optional | Array {
            podAffinityTerm | {
              labelSelector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%,
              matchLabelKeys | optional | Array String | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
              mismatchLabelKeys | optional | Array String | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
              namespaceSelector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%,
              namespaces | optional | Array String | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%,
              topologyKey | String | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%,
            } | doc "Required. A pod affinity term, associated with the corresponding weight.",
            weight | Number | doc m%"
weight associated with matching the corresponding podAffinityTerm,
in the range 1-100.
"%,
          } | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy
the anti-affinity expressions specified by this field, but it may choose
a node that violates one or more of the expressions. The node that is
most preferred is the one with the greatest sum of weights, i.e.
for each node that meets all of the scheduling requirements (resource
request, requiredDuringScheduling anti-affinity expressions, etc.),
compute a sum by iterating through the elements of this field and adding
"weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the
node(s) with the highest sum are the most preferred.
"%,
          requiredDuringSchedulingIgnoredDuringExecution | optional | Array {
            labelSelector | optional | {
              matchExpressions | optional | Array {
                key | String | doc "key is the label key that the selector applies to.",
                operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
              } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
              matchLabels | optional | {
              } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
            } | doc m%"
A label query over a set of resources, in this case pods.
If it's null, this PodAffinityTerm matches with no Pods.
"%,
            matchLabelKeys | optional | Array String | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both matchLabelKeys and labelSelector.
Also, matchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
            mismatchLabelKeys | optional | Array String | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will
be taken into consideration. The keys are used to lookup values from the
incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)`
to select the group of existing pods which pods will be taken into consideration
for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming
pod labels will be ignored. The default value is empty.
The same key is forbidden to exist in both mismatchLabelKeys and labelSelector.
Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
This is an alpha field and requires enabling MatchLabelKeysInPodAffinity feature gate.
"%,
            namespaceSelector | optional | {
              matchExpressions | optional | Array {
                key | String | doc "key is the label key that the selector applies to.",
                operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
              } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
              matchLabels | optional | {
              } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
            } | doc m%"
A label query over the set of namespaces that the term applies to.
The term is applied to the union of the namespaces selected by this field
and the ones listed in the namespaces field.
null selector and null or empty namespaces list means "this pod's namespace".
An empty selector ({}) matches all namespaces.
"%,
            namespaces | optional | Array String | doc m%"
namespaces specifies a static list of namespace names that the term applies to.
The term is applied to the union of the namespaces listed in this field
and the ones selected by namespaceSelector.
null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%,
            topologyKey | String | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching
the labelSelector in the specified namespaces, where co-located is defined as running on a node
whose value of the label with key topologyKey matches that of any node on which any of the
selected pods is running.
Empty topologyKey is not allowed.
"%,
          } | doc m%"
If the anti-affinity requirements specified by this field are not met at
scheduling time, the pod will not be scheduled onto the node.
If the anti-affinity requirements specified by this field cease to be met
at some point during pod execution (e.g. due to a pod label update), the
system may or may not try to eventually evict the pod from its node.
When there are multiple elements, the lists of nodes corresponding to each
podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%,
        } | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%,
      } | doc "If specified, the pod's scheduling constraints",
      args | optional | Array String | doc m%"
Arguments to the entrypoint.
The docker image's CMD is used if this is not provided.
Variable references $(VAR_NAME) are expanded using the container's environment. If a variable
cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax
can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
regardless of whether the variable exists or not.
Cannot be updated.
More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%,
      env | optional | Array k8s_io_objectmeta.EnvVar | doc m%"
List of environment variables to set in the container.
Cannot be updated.
"%,
      envFrom | optional | Array k8s_io_objectmeta.EnvFromSource | doc m%"
List of sources to populate environment variables in the container.
The keys defined within a source must be a C_IDENTIFIER. All invalid keys
will be reported as an event when the container is starting. When a key exists in multiple
sources, the value associated with the last source will take precedence.
Values defined by an Env with a duplicate key will take precedence.
Cannot be updated.
"%,
      image | optional | String | doc m%"
Docker image name.
More info: https://kubernetes.io/docs/concepts/containers/images
This field is optional to allow higher level config management to default or override
container images in workload controllers like Deployments and StatefulSets.
"%,
      imagePullPolicy | optional | String | doc m%"
Image pull policy.
One of Always, Never, IfNotPresent.
Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%,
      imagePullSecrets | optional | Array {
        name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
      } | doc m%"
ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
If specified, these secrets will be passed to individual puller implementations for them to use. For example,
in the case of docker, only DockerConfig type secrets are honored.
More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
Setting ImagePullSecrets will replace any secrets that have been
propagated to a controller Deployment, typically via packagePullSecrets.
"%,
      metadata | optional | {
        annotations | optional | {
        } | doc m%"
Annotations is an unstructured key value map stored with a resource that may be
set by external tools to store and retrieve arbitrary metadata. They are not
queryable and should be preserved when modifying objects.
More info: http:https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/
"%,
        labels | optional | {
        } | doc m%"
Map of string keys and values that can be used to organize and
categorize (scope and select) objects. This will only affect
labels on the pod, not the pod selector. Labels will be merged
with internal labels used by crossplane, and labels with a
crossplane.io key might be overwritten.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
"%,
      } | doc "Metadata that will be added to the provider Pod.",
      nodeName | optional | String | doc m%"
NodeName is a request to schedule this pod onto a specific node. If it is non-empty,
the scheduler simply schedules this pod onto that node, assuming that it fits resource
requirements.
"%,
      nodeSelector | optional | {
      } | doc m%"
NodeSelector is a selector which must be true for the pod to fit on a node.
Selector which must match a node's labels for the pod to be scheduled on that node.
More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
"%,
      podSecurityContext | optional | {
        appArmorProfile | optional | {
          localhostProfile | optional | String | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%,
          type | String | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%,
        } | doc m%"
appArmorProfile is the AppArmor options to use by the containers in this pod.
Note that this field cannot be set when spec.os.name is windows.
"%,
        fsGroup | optional | Number | doc m%"
A special supplemental group that applies to all containers in a pod.
Some volume types allow the Kubelet to change the ownership of that volume
to be owned by the pod:


1. The owning GID will be the FSGroup
2. The setgid bit is set (new files created in the volume will be owned by FSGroup)
3. The permission bits are OR'd with rw-rw----


If unset, the Kubelet will not modify the ownership and permissions of any volume.
Note that this field cannot be set when spec.os.name is windows.
"%,
        fsGroupChangePolicy | optional | String | doc m%"
fsGroupChangePolicy defines behavior of changing ownership and permission of the volume
before being exposed inside Pod. This field will only apply to
volume types which support fsGroup based ownership(and permissions).
It will have no effect on ephemeral volume types such as: secret, configmaps
and emptydir.
Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
Note that this field cannot be set when spec.os.name is windows.
"%,
        runAsGroup | optional | Number | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence
for that container.
Note that this field cannot be set when spec.os.name is windows.
"%,
        runAsNonRoot | optional | Bool | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%,
        runAsUser | optional | Number | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in SecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence
for that container.
Note that this field cannot be set when spec.os.name is windows.
"%,
        seLinuxOptions | optional | {
          level | optional | String | doc "Level is SELinux level label that applies to the container.",
          role | optional | String | doc "Role is a SELinux role label that applies to the container.",
          type | optional | String | doc "Type is a SELinux type label that applies to the container.",
          user | optional | String | doc "User is a SELinux user label that applies to the container.",
        } | doc m%"
The SELinux context to be applied to all containers.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in SecurityContext.  If set in
both SecurityContext and PodSecurityContext, the value specified in SecurityContext
takes precedence for that container.
Note that this field cannot be set when spec.os.name is windows.
"%,
        seccompProfile | optional | {
          localhostProfile | optional | String | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%,
          type | String | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:


Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%,
        } | doc m%"
The seccomp options to use by the containers in this pod.
Note that this field cannot be set when spec.os.name is windows.
"%,
        supplementalGroups | optional | Array Number | doc m%"
A list of groups applied to the first process run in each container, in addition
to the container's primary GID, the fsGroup (if specified), and group memberships
defined in the container image for the uid of the container process. If unspecified,
no additional groups are added to any container. Note that group memberships
defined in the container image for the uid of the container process are still effective,
even if they are not included in this list.
Note that this field cannot be set when spec.os.name is windows.
"%,
        sysctls | optional | Array {
          name | String | doc "Name of a property to set",
          value | String | doc "Value of a property to set",
        } | doc m%"
Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported
sysctls (by the container runtime) might fail to launch.
Note that this field cannot be set when spec.os.name is windows.
"%,
        windowsOptions | optional | {
          gmsaCredentialSpec | optional | String | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%,
          gmsaCredentialSpecName | optional | String | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use.",
          hostProcess | optional | Bool | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%,
          runAsUserName | optional | String | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%,
        } | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options within a container's SecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%,
      } | doc m%"
PodSecurityContext holds pod-level security attributes and common container settings.
Optional: Defaults to empty.  See type description for default values of each field.
"%,
      ports | optional | Array {
        containerPort | Number | doc m%"
Number of port to expose on the pod's IP address.
This must be a valid port number, 0 < x < 65536.
"%,
        hostIP | optional | String | doc "What host IP to bind the external port to.",
        hostPort | optional | Number | doc m%"
Number of port to expose on the host.
If specified, this must be a valid port number, 0 < x < 65536.
If HostNetwork is specified, this must match ContainerPort.
Most containers do not need this.
"%,
        name | optional | String | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each
named port in a pod must have a unique name. Name for the port that can be
referred to by services.
"%,
        protocol | String | doc m%"
Protocol for port. Must be UDP, TCP, or SCTP.
Defaults to "TCP".
"% | default = "TCP",
      } | doc "List of container ports to expose on the container",
      priorityClassName | optional | String | doc m%"
If specified, indicates the pod's priority. "system-node-critical" and
"system-cluster-critical" are two special keywords which indicate the
highest priorities with the former being the highest priority. Any other
name must be defined by creating a PriorityClass object with that name.
If not specified, the pod priority will be default or zero if there is no
default.
"%,
      replicas | optional | Number | doc m%"
Number of desired pods. This is a pointer to distinguish between explicit
zero and not specified. Defaults to 1.
Note: If more than 1 replica is set and leader election is not enabled then
controllers could conflict. Environment variable "LEADER_ELECTION" can be
used to enable leader election process.
"%,
      resources | optional | {
        claims | optional | Array {
          name | String | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of
the Pod where this field is used. It makes that resource available
inside a container.
"%,
        } | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims,
that are used by this container.


This is an alpha field and requires enabling the
DynamicResourceAllocation feature gate.


This field is immutable. It can only be set for containers.
"%,
        limits | optional | {
        } | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%,
        requests | optional | {
        } | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%,
      } | doc m%"
Compute Resources required by this container.
Cannot be updated.
More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
"%,
      runtimeClassName | optional | String | doc m%"
RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used
to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run.
If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an
empty definition that uses the default runtime handler.
More info: https://github.com/kubernetes/enhancements/blob/master/keps/sig-node/585-runtime-class/README.md
This is a beta feature as of Kubernetes v1.14.
"%,
      securityContext | optional | {
        allowPrivilegeEscalation | optional | Bool | doc m%"
AllowPrivilegeEscalation controls whether a process can gain more
privileges than its parent process. This bool directly controls if
the no_new_privs flag will be set on the container process.
AllowPrivilegeEscalation is true always when the container is:
1) run as Privileged
2) has CAP_SYS_ADMIN
Note that this field cannot be set when spec.os.name is windows.
"%,
        appArmorProfile | optional | {
          localhostProfile | optional | String | doc m%"
localhostProfile indicates a profile loaded on the node that should be used.
The profile must be preconfigured on the node to work.
Must match the loaded name of the profile.
Must be set if and only if type is "Localhost".
"%,
          type | String | doc m%"
type indicates which kind of AppArmor profile will be applied.
Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%,
        } | doc m%"
appArmorProfile is the AppArmor options to use by this container. If set, this profile
overrides the pod's appArmorProfile.
Note that this field cannot be set when spec.os.name is windows.
"%,
        capabilities | optional | {
          add | optional | Array String | doc "Added capabilities",
          drop | optional | Array String | doc "Removed capabilities",
        } | doc m%"
The capabilities to add/drop when running containers.
Defaults to the default set of capabilities granted by the container runtime.
Note that this field cannot be set when spec.os.name is windows.
"%,
        privileged | optional | Bool | doc m%"
Run container in privileged mode.
Processes in privileged containers are essentially equivalent to root on the host.
Defaults to false.
Note that this field cannot be set when spec.os.name is windows.
"%,
        procMount | optional | String | doc m%"
procMount denotes the type of proc mount to use for the containers.
The default is DefaultProcMount which uses the container runtime defaults for
readonly paths and masked paths.
This requires the ProcMountType feature flag to be enabled.
Note that this field cannot be set when spec.os.name is windows.
"%,
        readOnlyRootFilesystem | optional | Bool | doc m%"
Whether this container has a read-only root filesystem.
Default is false.
Note that this field cannot be set when spec.os.name is windows.
"%,
        runAsGroup | optional | Number | doc m%"
The GID to run the entrypoint of the container process.
Uses runtime default if unset.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%,
        runAsNonRoot | optional | Bool | doc m%"
Indicates that the container must run as a non-root user.
If true, the Kubelet will validate the image at runtime to ensure that it
does not run as UID 0 (root) and fail to start the container if it does.
If unset or false, no such validation will be performed.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%,
        runAsUser | optional | Number | doc m%"
The UID to run the entrypoint of the container process.
Defaults to user specified in image metadata if unspecified.
May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%,
        seLinuxOptions | optional | {
          level | optional | String | doc "Level is SELinux level label that applies to the container.",
          role | optional | String | doc "Role is a SELinux role label that applies to the container.",
          type | optional | String | doc "Type is a SELinux type label that applies to the container.",
          user | optional | String | doc "User is a SELinux user label that applies to the container.",
        } | doc m%"
The SELinux context to be applied to the container.
If unspecified, the container runtime will allocate a random SELinux context for each
container.  May also be set in PodSecurityContext.  If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is windows.
"%,
        seccompProfile | optional | {
          localhostProfile | optional | String | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used.
The profile must be preconfigured on the node to work.
Must be a descending path, relative to the kubelet's configured seccomp profile location.
Must be set if type is "Localhost". Must NOT be set for any other type.
"%,
          type | String | doc m%"
type indicates which kind of seccomp profile will be applied.
Valid options are:


Localhost - a profile defined in a file on the node should be used.
RuntimeDefault - the container runtime default profile should be used.
Unconfined - no profile should be applied.
"%,
        } | doc m%"
The seccomp options to use by this container. If seccomp options are
provided at both the pod & container level, the container options
override the pod options.
Note that this field cannot be set when spec.os.name is windows.
"%,
        windowsOptions | optional | {
          gmsaCredentialSpec | optional | String | doc m%"
GMSACredentialSpec is where the GMSA admission webhook
(https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the
GMSA credential spec named by the GMSACredentialSpecName field.
"%,
          gmsaCredentialSpecName | optional | String | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use.",
          hostProcess | optional | Bool | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container.
All of a Pod's containers must have the same effective HostProcess value
(it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).
In addition, if HostProcess is true then HostNetwork must also be set to true.
"%,
          runAsUserName | optional | String | doc m%"
The UserName in Windows to run the entrypoint of the container process.
Defaults to the user specified in image metadata if unspecified.
May also be set in PodSecurityContext. If set in both SecurityContext and
PodSecurityContext, the value specified in SecurityContext takes precedence.
"%,
        } | doc m%"
The Windows specific settings applied to all containers.
If unspecified, the options from the PodSecurityContext will be used.
If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
Note that this field cannot be set when spec.os.name is linux.
"%,
      } | doc m%"
SecurityContext holds container-level security attributes and common container settings.
Optional: Defaults to empty.  See type description for default values of each field.
"%,
      serviceAccountName | optional | String | doc m%"
ServiceAccountName is the name of the ServiceAccount to use to run this pod.
More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
If specified, a ServiceAccount named this ServiceAccountName will be used for
the spec.serviceAccountName field in Pods to be created and for the subjects.name field
in a ClusterRoleBinding to be created.
If there is no ServiceAccount named this ServiceAccountName, a new ServiceAccount
will be created.
If there is a pre-existing ServiceAccount named this ServiceAccountName, the ServiceAccount
will be used. The annotations in the ControllerConfig will be copied to the ServiceAccount
and pre-existing annotations will be kept.
Regardless of whether there is a ServiceAccount created by Crossplane or is in place already,
the ServiceAccount will be deleted once the Provider and ControllerConfig are deleted.
"%,
      tolerations | optional | Array k8s_io_objectmeta.Toleration | doc "If specified, the pod's tolerations.",
      volumeMounts | optional | Array k8s_io_objectmeta.VolumeMount | doc m%"
List of VolumeMounts to mount into the container's filesystem.
Cannot be updated.
"%,
      volumes | optional | Array {
        awsElasticBlockStore | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
TODO: how do we prevent errors in the filesystem from compromising the machine
"%,
          partition | optional | Number | doc m%"
partition is the partition in the volume that you want to mount.
If omitted, the default is to mount by volume name.
Examples: For volume /dev/sda1, you specify the partition as "1".
Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
"%,
          readOnly | optional | Bool | doc m%"
readOnly value true will force the readOnly setting in VolumeMounts.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%,
          volumeID | String | doc m%"
volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume).
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%,
        } | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%,
        azureDisk | optional | {
          cachingMode | optional | String | doc "cachingMode is the Host Caching mode: None, Read Only, Read Write.",
          diskName | String | doc "diskName is the Name of the data disk in the blob storage",
          diskURI | String | doc "diskURI is the URI of data disk in the blob storage",
          fsType | optional | String | doc m%"
fsType is Filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
          kind | optional | String | doc m%"
kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
"%,
          readOnly | optional | Bool | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
        } | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
"%,
        azureFile | optional | {
          readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          secretName | String | doc m%"
secretName is the  name of secret that contains Azure Storage Account Name and Key
"%,
          shareName | String | doc "shareName is the azure share Name",
        } | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod.
"%,
        cephfs | optional | {
          monitors | Array String | doc m%"
monitors is Required: Monitors is a collection of Ceph monitors
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
          path | optional | String | doc m%"
path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
"%,
          readOnly | optional | Bool | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
          secretFile | optional | String | doc m%"
secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty.
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
          user | optional | String | doc m%"
user is optional: User is the rados user name, default is admin
More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%,
        } | doc "cephFS represents a Ceph FS mount on the host that shares a pod's lifetime",
        cinder | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
          readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef is optional: points to a secret object containing parameters used to connect
to OpenStack.
"%,
          volumeID | String | doc m%"
volumeID used to identify the volume in cinder.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
        } | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine.
More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%,
        configMap | optional | {
          defaultMode | optional | Number | doc m%"
defaultMode is optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
          items | optional | Array {
            key | String | doc "key is the key to project.",
            mode | optional | Number | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
            path | String | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%,
          } | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
ConfigMap will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the ConfigMap,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%,
          name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          optional | optional | Bool | doc "optional specify whether the ConfigMap or its keys must be defined",
        } | doc "configMap represents a configMap that should populate this volume",
        csi | optional | {
          driver | String | doc m%"
driver is the name of the CSI driver that handles this volume.
Consult with your admin for the correct name as registered in the cluster.
"%,
          fsType | optional | String | doc m%"
fsType to mount. Ex. "ext4", "xfs", "ntfs".
If not provided, the empty value is passed to the associated CSI driver
which will determine the default filesystem to apply.
"%,
          nodePublishSecretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
nodePublishSecretRef is a reference to the secret object containing
sensitive information to pass to the CSI driver to complete the CSI
NodePublishVolume and NodeUnpublishVolume calls.
This field is optional, and  may be empty if no secret is required. If the
secret object contains more than one secret, all secret references are passed.
"%,
          readOnly | optional | Bool | doc m%"
readOnly specifies a read-only configuration for the volume.
Defaults to false (read/write).
"%,
          volumeAttributes | optional | {
          } | doc m%"
volumeAttributes stores driver-specific properties that are passed to the CSI
driver. Consult your driver's documentation for supported values.
"%,
        } | doc m%"
csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
"%,
        downwardAPI | optional | {
          defaultMode | optional | Number | doc m%"
Optional: mode bits to use on created files by default. Must be a
Optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
          items | optional | Array {
            fieldRef | optional | {
              apiVersion | optional | String | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\".",
              fieldPath | String | doc "Path of the field to select in the specified API version.",
            } | doc m%"
Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
"%,
            mode | optional | Number | doc m%"
Optional: mode bits used to set permissions on this file, must be an octal value
between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
            path | String | doc m%"
Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
"%,
            resourceFieldRef | optional | {
              containerName | optional | String | doc "Container name: required for volumes, optional for env vars",
              divisor | optional | Number | String | doc "Specifies the output format of the exposed resources, defaults to \"1\"",
              resource | String | doc "Required: resource to select",
            } | doc m%"
Selects a resource of the container: only resources limits and requests
(limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
"%,
          } | doc "Items is a list of downward API volume file",
        } | doc m%"
downwardAPI represents downward API about the pod that should populate this volume
"%,
        emptyDir | optional | {
          medium | optional | String | doc m%"
medium represents what type of storage medium should back this directory.
The default is "" which means to use the node's default medium.
Must be an empty string (default) or Memory.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%,
          sizeLimit | optional | Number | String | doc m%"
sizeLimit is the total amount of local storage required for this EmptyDir volume.
The size limit is also applicable for memory medium.
The maximum usage on memory medium EmptyDir would be the minimum value between
the SizeLimit specified here and the sum of memory limits of all containers in a pod.
The default is nil which means that the limit is undefined.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%,
        } | doc m%"
emptyDir represents a temporary directory that shares a pod's lifetime.
More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%,
        ephemeral | optional | {
          volumeClaimTemplate | optional | {
            metadata | optional | {
              annotations | optional | {
              },
              finalizers | optional | Array String,
              labels | optional | {
              },
              name | optional | String,
              namespace | optional | String,
            } | doc m%"
May contain labels and annotations that will be copied into the PVC
when creating it. No other fields are allowed and will be rejected during
validation.
"%,
            spec | {
              accessModes | optional | Array String | doc m%"
accessModes contains the desired access modes the volume should have.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
"%,
              dataSource | optional | {
                apiGroup | optional | String | doc m%"
APIGroup is the group for the resource being referenced.
If APIGroup is not specified, the specified Kind must be in the core API group.
For any other third-party types, APIGroup is required.
"%,
                kind | String | doc "Kind is the type of resource being referenced",
                name | String | doc "Name is the name of resource being referenced",
              } | doc m%"
dataSource field can be used to specify either:
* An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot)
* An existing PVC (PersistentVolumeClaim)
If the provisioner or an external controller can support the specified data source,
it will create a new volume based on the contents of the specified data source.
When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef,
and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified.
If the namespace is specified, then dataSourceRef will not be copied to dataSource.
"%,
              dataSourceRef | optional | {
                apiGroup | optional | String | doc m%"
APIGroup is the group for the resource being referenced.
If APIGroup is not specified, the specified Kind must be in the core API group.
For any other third-party types, APIGroup is required.
"%,
                kind | String | doc "Kind is the type of resource being referenced",
                name | String | doc "Name is the name of resource being referenced",
                namespace | optional | String | doc m%"
Namespace is the namespace of resource being referenced
Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details.
(Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%,
              } | doc m%"
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
"%,
              resources | optional | {
                limits | optional | {
                } | doc m%"
Limits describes the maximum amount of compute resources allowed.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%,
                requests | optional | {
                } | doc m%"
Requests describes the minimum amount of compute resources required.
If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
otherwise to an implementation-defined value. Requests cannot exceed Limits.
More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%,
              } | doc m%"
resources represents the minimum resources the volume should have.
If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements
that are lower than previous value but must still be higher than capacity recorded in the
status field of the claim.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
"%,
              selector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc "selector is a label query over volumes to consider for binding.",
              storageClassName | optional | String | doc m%"
storageClassName is the name of the StorageClass required by the claim.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
"%,
              volumeAttributesClassName | optional | String | doc m%"
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
(Alpha) Using this field requires the VolumeAttributesClass feature gate to be enabled.
"%,
              volumeMode | optional | String | doc m%"
volumeMode defines what type of volume is required by the claim.
Value of Filesystem is implied when not included in claim spec.
"%,
              volumeName | optional | String | doc "volumeName is the binding reference to the PersistentVolume backing this claim.",
            } | doc m%"
The specification for the PersistentVolumeClaim. The entire content is
copied unchanged into the PVC that gets created from this
template. The same fields as in a PersistentVolumeClaim
are also valid here.
"%,
          } | doc m%"
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
"%,
        } | doc m%"
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
"%,
        fc | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
TODO: how do we prevent errors in the filesystem from compromising the machine
"%,
          lun | optional | Number | doc "lun is Optional: FC target lun number",
          readOnly | optional | Bool | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          targetWWNs | optional | Array String | doc "targetWWNs is Optional: FC target worldwide names (WWNs)",
          wwids | optional | Array String | doc m%"
wwids Optional: FC volume world wide identifiers (wwids)
Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
"%,
        } | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%,
        flexVolume | optional | {
          driver | String | doc "driver is the name of the driver to use for this volume.",
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%,
          options | optional | {
          } | doc "options is Optional: this field holds extra command options if any.",
          readOnly | optional | Bool | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef is Optional: secretRef is reference to the secret object containing
sensitive information to pass to the plugin scripts. This may be
empty if no secret object is specified. If the secret object
contains more than one secret, all secrets are passed to the plugin
scripts.
"%,
        } | doc m%"
flexVolume represents a generic volume resource that is
provisioned/attached using an exec based plugin.
"%,
        flocker | optional | {
          datasetName | optional | String | doc m%"
datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker
should be considered as deprecated
"%,
          datasetUUID | optional | String | doc m%"
datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
"%,
        } | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
"%,
        gcePersistentDisk | optional | {
          fsType | optional | String | doc m%"
fsType is filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
TODO: how do we prevent errors in the filesystem from compromising the machine
"%,
          partition | optional | Number | doc m%"
partition is the partition in the volume that you want to mount.
If omitted, the default is to mount by volume name.
Examples: For volume /dev/sda1, you specify the partition as "1".
Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
          pdName | String | doc m%"
pdName is unique name of the PD resource in GCE. Used to identify the disk in GCE.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
        } | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
        gitRepo | optional | {
          directory | optional | String | doc m%"
directory is the target directory name.
Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the
git repository.  Otherwise, if specified, the volume will contain the git repository in
the subdirectory with the given name.
"%,
          repository | String | doc "repository is the URL",
          revision | optional | String | doc "revision is the commit hash for the specified revision.",
        } | doc m%"
gitRepo represents a git repository at a particular revision.
DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an
EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir
into the Pod's container.
"%,
        glusterfs | optional | {
          endpoints | String | doc m%"
endpoints is the endpoint name that details Glusterfs topology.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
          path | String | doc m%"
path is the Glusterfs volume path.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the Glusterfs volume to be mounted with read-only permissions.
Defaults to false.
More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
        } | doc m%"
glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime.
More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%,
        hostPath | optional | {
          path | String | doc m%"
path of the directory on the host.
If the path is a symlink, it will follow the link to the real path.
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
          type | optional | String | doc m%"
type for HostPath Volume
Defaults to ""
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
        } | doc m%"
hostPath represents a pre-existing file or directory on the host
machine that is directly exposed to the container. This is generally
used for system agents or other privileged things that are allowed
to see the host machine. Most containers will NOT need this.
More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
---
TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not
mount host directories as read/write.
"%,
        iscsi | optional | {
          chapAuthDiscovery | optional | Bool | doc "chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication",
          chapAuthSession | optional | Bool | doc "chapAuthSession defines whether support iSCSI Session CHAP authentication",
          fsType | optional | String | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
TODO: how do we prevent errors in the filesystem from compromising the machine
"%,
          initiatorName | optional | String | doc m%"
initiatorName is the custom iSCSI Initiator Name.
If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface
<target portal>:<volume name> will be created for the connection.
"%,
          iqn | String | doc "iqn is the target iSCSI Qualified Name.",
          iscsiInterface | optional | String | doc m%"
iscsiInterface is the interface Name that uses an iSCSI transport.
Defaults to 'default' (tcp).
"%,
          lun | Number | doc "lun represents iSCSI Target Lun number.",
          portals | optional | Array String | doc m%"
portals is the iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port
is other than default (typically TCP ports 860 and 3260).
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc "secretRef is the CHAP Secret for iSCSI target and initiator authentication",
          targetPortal | String | doc m%"
targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port
is other than default (typically TCP ports 860 and 3260).
"%,
        } | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a
kubelet's host machine and then exposed to the pod.
More info: https://examples.k8s.io/volumes/iscsi/README.md
"%,
        name | String | doc m%"
name of the volume.
Must be a DNS_LABEL and unique within the pod.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%,
        nfs | optional | {
          path | String | doc m%"
path that is exported by the NFS server.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the NFS export to be mounted with read-only permissions.
Defaults to false.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
          server | String | doc m%"
server is the hostname or IP address of the NFS server.
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
        } | doc m%"
nfs represents an NFS mount on the host that shares a pod's lifetime
More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
        persistentVolumeClaim | optional | {
          claimName | String | doc m%"
claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%,
          readOnly | optional | Bool | doc m%"
readOnly Will force the ReadOnly setting in VolumeMounts.
Default false.
"%,
        } | doc m%"
persistentVolumeClaimVolumeSource represents a reference to a
PersistentVolumeClaim in the same namespace.
More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%,
        photonPersistentDisk | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
          pdID | String | doc "pdID is the ID that identifies Photon Controller persistent disk",
        } | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
"%,
        portworxVolume | optional | {
          fsType | optional | String | doc m%"
fSType represents the filesystem type to mount
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
"%,
          readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          volumeID | String | doc "volumeID uniquely identifies a Portworx volume",
        } | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine
"%,
        projected | optional | {
          defaultMode | optional | Number | doc m%"
defaultMode are the mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
          sources | optional | Array {
            clusterTrustBundle | optional | {
              labelSelector | optional | {
                matchExpressions | optional | Array {
                  key | String | doc "key is the label key that the selector applies to.",
                  operator | String | doc m%"
operator represents a key's relationship to a set of values.
Valid operators are In, NotIn, Exists and DoesNotExist.
"%,
                  values | optional | Array String | doc m%"
values is an array of string values. If the operator is In or NotIn,
the values array must be non-empty. If the operator is Exists or DoesNotExist,
the values array must be empty. This array is replaced during a strategic
merge patch.
"%,
                } | doc m%"
matchExpressions is a list of label selector requirements. The requirements are ANDed.
"%,
                matchLabels | optional | {
                } | doc m%"
matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
map is equivalent to an element of matchExpressions, whose key field is "key", the
operator is "In", and the values array contains only "value". The requirements are ANDed.
"%,
              } | doc m%"
Select all ClusterTrustBundles that match this label selector.  Only has
effect if signerName is set.  Mutually-exclusive with name.  If unset,
interpreted as "match nothing".  If set but empty, interpreted as "match
everything".
"%,
              name | optional | String | doc m%"
Select a single ClusterTrustBundle by object name.  Mutually-exclusive
with signerName and labelSelector.
"%,
              optional | optional | Bool | doc m%"
If true, don't block pod startup if the referenced ClusterTrustBundle(s)
aren't available.  If using name, then the named ClusterTrustBundle is
allowed not to exist.  If using signerName, then the combination of
signerName and labelSelector is allowed to match zero
ClusterTrustBundles.
"%,
              path | String | doc "Relative path from the volume root to write the bundle.",
              signerName | optional | String | doc m%"
Select all ClusterTrustBundles that match this signer name.
Mutually-exclusive with name.  The contents of all selected
ClusterTrustBundles will be unified and deduplicated.
"%,
            } | doc m%"
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
"%,
            configMap | optional | {
              items | optional | Array {
                key | String | doc "key is the key to project.",
                mode | optional | Number | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
                path | String | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%,
              } | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
ConfigMap will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the ConfigMap,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%,
              name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
              optional | optional | Bool | doc "optional specify whether the ConfigMap or its keys must be defined",
            } | doc "configMap information about the configMap data to project",
            downwardAPI | optional | {
              items | optional | Array {
                fieldRef | optional | {
                  apiVersion | optional | String | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\".",
                  fieldPath | String | doc "Path of the field to select in the specified API version.",
                } | doc m%"
Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
"%,
                mode | optional | Number | doc m%"
Optional: mode bits used to set permissions on this file, must be an octal value
between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
                path | String | doc m%"
Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
"%,
                resourceFieldRef | optional | {
                  containerName | optional | String | doc "Container name: required for volumes, optional for env vars",
                  divisor | optional | Number | String | doc "Specifies the output format of the exposed resources, defaults to \"1\"",
                  resource | String | doc "Required: resource to select",
                } | doc m%"
Selects a resource of the container: only resources limits and requests
(limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
"%,
              } | doc "Items is a list of DownwardAPIVolume file",
            } | doc "downwardAPI information about the downwardAPI data to project",
            secret | optional | {
              items | optional | Array {
                key | String | doc "key is the key to project.",
                mode | optional | Number | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
                path | String | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%,
              } | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced
Secret will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the Secret,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%,
              name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
              optional | optional | Bool | doc "optional field specify whether the Secret or its key must be defined",
            } | doc "secret information about the secret data to project",
            serviceAccountToken | optional | {
              audience | optional | String | doc m%"
audience is the intended audience of the token. A recipient of a token
must identify itself with an identifier specified in the audience of the
token, and otherwise should reject the token. The audience defaults to the
identifier of the apiserver.
"%,
              expirationSeconds | optional | Number | doc m%"
expirationSeconds is the requested duration of validity of the service
account token. As the token approaches expiration, the kubelet volume
plugin will proactively rotate the service account token. The kubelet will
start trying to rotate the token if the token is older than 80 percent of
its time to live or if the token is older than 24 hours.Defaults to 1 hour
and must be at least 10 minutes.
"%,
              path | String | doc m%"
path is the path relative to the mount point of the file to project the
token into.
"%,
            } | doc "serviceAccountToken is information about the serviceAccountToken data to project",
          } | doc "sources is the list of volume projections",
        } | doc "projected items for all in one resources secrets, configmaps, and downward API",
        quobyte | optional | {
          group | optional | String | doc m%"
group to map volume access to
Default is no group
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the Quobyte volume to be mounted with read-only permissions.
Defaults to false.
"%,
          registry | String | doc m%"
registry represents a single or multiple Quobyte Registry services
specified as a string as host:port pair (multiple entries are separated with commas)
which acts as the central registry for volumes
"%,
          tenant | optional | String | doc m%"
tenant owning the given Quobyte volume in the Backend
Used with dynamically provisioned Quobyte volumes, value is set by the plugin
"%,
          user | optional | String | doc m%"
user to map volume access to
Defaults to serivceaccount user
"%,
          volume | String | doc "volume is a string that references an already created Quobyte volume by name.",
        } | doc "quobyte represents a Quobyte mount on the host that shares a pod's lifetime",
        rbd | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type of the volume that you want to mount.
Tip: Ensure that the filesystem type is supported by the host operating system.
Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
TODO: how do we prevent errors in the filesystem from compromising the machine
"%,
          image | String | doc m%"
image is the rados image name.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          keyring | optional | String | doc m%"
keyring is the path to key ring for RBDUser.
Default is /etc/ceph/keyring.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          monitors | Array String | doc m%"
monitors is a collection of Ceph monitors.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          pool | optional | String | doc m%"
pool is the rados pool name.
Default is rbd.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          readOnly | optional | Bool | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts.
Defaults to false.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef is name of the authentication secret for RBDUser. If provided
overrides keyring.
Default is nil.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
          user | optional | String | doc m%"
user is the rados user name.
Default is admin.
More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
        } | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime.
More info: https://examples.k8s.io/volumes/rbd/README.md
"%,
        scaleIO | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs".
Default is "xfs".
"%,
          gateway | String | doc "gateway is the host address of the ScaleIO API Gateway.",
          protectionDomain | optional | String | doc m%"
protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
"%,
          readOnly | optional | Bool | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          secretRef | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef references to the secret for ScaleIO user and other
sensitive information. If this is not provided, Login operation will fail.
"%,
          sslEnabled | optional | Bool | doc "sslEnabled Flag enable/disable SSL communication with Gateway, default false",
          storageMode | optional | String | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned.
Default is ThinProvisioned.
"%,
          storagePool | optional | String | doc "storagePool is the ScaleIO Storage Pool associated with the protection domain.",
          system | String | doc "system is the name of the storage system as configured in ScaleIO.",
          volumeName | optional | String | doc m%"
volumeName is the name of a volume already created in the ScaleIO system
that is associated with this volume source.
"%,
        } | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
"%,
        secret | optional | {
          defaultMode | optional | Number | doc m%"
defaultMode is Optional: mode bits used to set permissions on created files by default.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values
for mode bits. Defaults to 0644.
Directories within the path are not affected by this setting.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
          items | optional | Array {
            key | String | doc "key is the key to project.",
            mode | optional | Number | doc m%"
mode is Optional: mode bits used to set permissions on this file.
Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511.
YAML accepts both octal and decimal values, JSON requires decimal values for mode bits.
If not specified, the volume defaultMode will be used.
This might be in conflict with other options that affect the file
mode, like fsGroup, and the result can be other mode bits set.
"%,
            path | String | doc m%"
path is the relative path of the file to map the key to.
May not be an absolute path.
May not contain the path element '..'.
May not start with the string '..'.
"%,
          } | doc m%"
items If unspecified, each key-value pair in the Data field of the referenced
Secret will be projected into the volume as a file whose name is the
key and content is the value. If specified, the listed keys will be
projected into the specified paths, and unlisted keys will not be
present. If a key is specified which is not present in the Secret,
the volume setup will error unless it is marked optional. Paths must be
relative and may not contain the '..' path or start with '..'.
"%,
          optional | optional | Bool | doc "optional field specify whether the Secret or its keys must be defined",
          secretName | optional | String | doc m%"
secretName is the name of the secret in the pod's namespace to use.
More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%,
        } | doc m%"
secret represents a secret that should populate this volume.
More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%,
        storageos | optional | {
          fsType | optional | String | doc m%"
fsType is the filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
          readOnly | optional | Bool | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force
the ReadOnly setting in VolumeMounts.
"%,
          secretRef | optional | {
            name | optional | String | doc m%"
Name of the referent.
More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
TODO: Add other useful fields. apiVersion, kind, uid?
"%,
          } | doc m%"
secretRef specifies the secret to use for obtaining the StorageOS API
credentials.  If not specified, default values will be attempted.
"%,
          volumeName | optional | String | doc m%"
volumeName is the human-readable name of the StorageOS volume.  Volume
names are only unique within a namespace.
"%,
          volumeNamespace | optional | String | doc m%"
volumeNamespace specifies the scope of the volume within StorageOS.  If no
namespace is specified then the Pod's namespace will be used.  This allows the
Kubernetes name scoping to be mirrored within StorageOS for tighter integration.
Set VolumeName to any name to override the default behaviour.
Set to "default" if you are not using namespaces within StorageOS.
Namespaces that do not pre-exist within StorageOS will be created.
"%,
        } | doc m%"
storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
"%,
        vsphereVolume | optional | {
          fsType | optional | String | doc m%"
fsType is filesystem type to mount.
Must be a filesystem type supported by the host operating system.
Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%,
          storagePolicyID | optional | String | doc m%"
storagePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
"%,
          storagePolicyName | optional | String | doc "storagePolicyName is the storage Policy Based Management (SPBM) profile name.",
          volumePath | String | doc "volumePath is the path that identifies vSphere volume vmdk",
        } | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
"%,
      } | doc m%"
List of volumes that can be mounted by containers belonging to the pod.
More info: https://kubernetes.io/docs/concepts/storage/volumes
"%,
    } | doc m%"
ControllerConfigSpec specifies the configuration for a packaged controller.
Values provided will override package manager defaults. Labels and
annotations are passed to both the controller Deployment and ServiceAccount.
"%,
  },
}
