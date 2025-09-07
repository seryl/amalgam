{
  ControllerRevision = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    data
       | RawExtension
       | doc "Data is the serialized representation of the state."
       | optional,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ObjectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    revision
       | Number
       | doc "Revision indicates the revision of the state represented by Data."
  },

  ControllerRevisionList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apps.v1.ControllerRevision
       | doc "Items is the list of ControllerRevisions",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc m%"
More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  DaemonSet = {
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
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.apps.v1.DaemonSetSpec
       | doc m%"
The desired behavior of this daemon set. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | io.k8s.api.apps.v1.DaemonSetStatus
       | doc m%"
The current status of this daemon set. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  DaemonSetCondition = {
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "A human readable message indicating details about the transition."
       | optional,
    reason
       | String
       | doc "The reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of DaemonSet condition."
  },

  DaemonSetList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apps.v1.DaemonSet
       | doc "A list of daemon sets.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  DaemonSetSpec = {
    minReadySeconds
       | Number
       | doc m%"
The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
"%
       | optional,
    revisionHistoryLimit
       | Number
       | doc m%"
The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
"%
       | optional,
    selector
       | LabelSelector
       | doc m%"
A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
"%,
    template
       | PodTemplateSpec
       | doc m%"
An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). The only allowed template.spec.restartPolicy value is "Always". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
"%,
    updateStrategy
       | io.k8s.api.apps.v1.DaemonSetUpdateStrategy
       | doc "An update strategy to replace existing DaemonSet pods with new pods."
       | optional
  },

  DaemonSetStatus = {
    collisionCount
       | Number
       | doc m%"
Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
"%
       | optional,
    conditions
       | Array io.k8s.api.apps.v1.DaemonSetCondition
       | doc "Represents the latest available observations of a DaemonSet's current state."
       | optional,
    currentNumberScheduled
       | Number
       | doc m%"
The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
    desiredNumberScheduled
       | Number
       | doc m%"
The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
    numberAvailable
       | Number
       | doc m%"
The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
"%
       | optional,
    numberMisscheduled
       | Number
       | doc m%"
The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
"%,
    numberReady
       | Number
       | doc m%"
numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running with a Ready Condition.
"%,
    numberUnavailable
       | Number
       | doc m%"
The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
"%
       | optional,
    observedGeneration
       | Number
       | doc "The most recent generation observed by the daemon set controller."
       | optional,
    updatedNumberScheduled
       | Number
       | doc "The total number of nodes that are running updated daemon pod"
       | optional
  },

  DaemonSetUpdateStrategy = {
    rollingUpdate
       | io.k8s.api.apps.v1.RollingUpdateDaemonSet
       | doc "Rolling update config params. Present only if type = \"RollingUpdate\"."
       | optional,
    type
       | String
       | doc m%"
Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
"%
       | optional
  },

  Deployment = {
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
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.apps.v1.DeploymentSpec
       | doc "Specification of the desired behavior of the Deployment."
       | optional,
    status
       | io.k8s.api.apps.v1.DeploymentStatus
       | doc "Most recently observed status of the Deployment."
       | optional
  },

  DeploymentCondition = {
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    lastUpdateTime
       | Time
       | doc "The last time this condition was updated."
       | optional,
    message
       | String
       | doc "A human readable message indicating details about the transition."
       | optional,
    reason
       | String
       | doc "The reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of deployment condition."
  },

  DeploymentList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apps.v1.Deployment
       | doc "Items is the list of Deployments.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc "Standard list metadata."
       | optional
  },

  DeploymentSpec = {
    minReadySeconds
       | Number
       | doc m%"
Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
"%
       | optional,
    paused
       | Bool
       | doc "Indicates that the deployment is paused."
       | optional,
    progressDeadlineSeconds
       | Number
       | doc m%"
The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s.
"%
       | optional,
    replicas
       | Number
       | doc m%"
Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
"%
       | optional,
    revisionHistoryLimit
       | Number
       | doc m%"
The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
"%
       | optional,
    selector
       | LabelSelector
       | doc m%"
Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template's labels.
"%,
    strategy
       | io.k8s.api.apps.v1.DeploymentStrategy
       | doc "The deployment strategy to use to replace existing pods with new ones."
       | optional,
    template
       | PodTemplateSpec
       | doc m%"
Template describes the pods that will be created. The only allowed template.spec.restartPolicy value is "Always".
"%
  },

  DeploymentStatus = {
    availableReplicas
       | Number
       | doc m%"
Total number of available non-terminating pods (ready for at least minReadySeconds) targeted by this deployment.
"%
       | optional,
    collisionCount
       | Number
       | doc m%"
Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
"%
       | optional,
    conditions
       | Array io.k8s.api.apps.v1.DeploymentCondition
       | doc "Represents the latest available observations of a deployment's current state."
       | optional,
    observedGeneration
       | Number
       | doc "The generation observed by the deployment controller."
       | optional,
    readyReplicas
       | Number
       | doc m%"
Total number of non-terminating pods targeted by this Deployment with a Ready Condition.
"%
       | optional,
    replicas
       | Number
       | doc m%"
Total number of non-terminating pods targeted by this deployment (their labels match the selector).
"%
       | optional,
    terminatingReplicas
       | Number
       | doc m%"
Total number of terminating pods targeted by this deployment. Terminating pods have a non-null .metadata.deletionTimestamp and have not yet reached the Failed or Succeeded .status.phase.

This is an alpha field. Enable DeploymentReplicaSetTerminatingReplicas to be able to use this field.
"%
       | optional,
    unavailableReplicas
       | Number
       | doc m%"
Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
"%
       | optional,
    updatedReplicas
       | Number
       | doc m%"
Total number of non-terminating pods targeted by this deployment that have the desired template spec.
"%
       | optional
  },

  DeploymentStrategy = {
    rollingUpdate
       | io.k8s.api.apps.v1.RollingUpdateDeployment
       | doc m%"
Rolling update config params. Present only if DeploymentStrategyType = RollingUpdate.
"%
       | optional,
    type
       | String
       | doc m%"
Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
"%
       | optional
  },

  ReplicaSet = {
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
       | doc m%"
If the Labels of a ReplicaSet are empty, they are defaulted to be the same as the Pod(s) that the ReplicaSet manages. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.apps.v1.ReplicaSetSpec
       | doc m%"
Spec defines the specification of the desired behavior of the ReplicaSet. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | io.k8s.api.apps.v1.ReplicaSetStatus
       | doc m%"
Status is the most recently observed status of the ReplicaSet. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  ReplicaSetCondition = {
    lastTransitionTime
       | Time
       | doc "The last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "A human readable message indicating details about the transition."
       | optional,
    reason
       | String
       | doc "The reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of replica set condition."
  },

  ReplicaSetList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apps.v1.ReplicaSet
       | doc m%"
List of ReplicaSets. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset
"%,
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc m%"
Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional
  },

  ReplicaSetSpec = {
    minReadySeconds
       | Number
       | doc m%"
Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
"%
       | optional,
    replicas
       | Number
       | doc m%"
Replicas is the number of desired pods. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset
"%
       | optional,
    selector
       | LabelSelector
       | doc m%"
Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
"%,
    template
       | PodTemplateSpec
       | doc m%"
Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset/#pod-template
"%
       | optional
  },

  ReplicaSetStatus = {
    availableReplicas
       | Number
       | doc m%"
The number of available non-terminating pods (ready for at least minReadySeconds) for this replica set.
"%
       | optional,
    conditions
       | Array io.k8s.api.apps.v1.ReplicaSetCondition
       | doc "Represents the latest available observations of a replica set's current state."
       | optional,
    fullyLabeledReplicas
       | Number
       | doc m%"
The number of non-terminating pods that have labels matching the labels of the pod template of the replicaset.
"%
       | optional,
    observedGeneration
       | Number
       | doc m%"
ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
"%
       | optional,
    readyReplicas
       | Number
       | doc m%"
The number of non-terminating pods targeted by this ReplicaSet with a Ready Condition.
"%
       | optional,
    replicas
       | Number
       | doc m%"
Replicas is the most recently observed number of non-terminating pods. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicaset
"%,
    terminatingReplicas
       | Number
       | doc m%"
The number of terminating pods for this replica set. Terminating pods have a non-null .metadata.deletionTimestamp and have not yet reached the Failed or Succeeded .status.phase.

This is an alpha field. Enable DeploymentReplicaSetTerminatingReplicas to be able to use this field.
"%
       | optional
  },

  RollingUpdateDaemonSet = {
    maxSurge
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
The maximum number of nodes with an existing available DaemonSet pod that can have an updated DaemonSet pod during during an update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up to a minimum of 1. Default value is 0. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their a new pod created before the old pod is marked as deleted. The update starts by launching new pods on 30% of nodes. Once an updated pod is available (Ready for at least minReadySeconds) the old DaemonSet pod on that node is marked deleted. If the old pod becomes unavailable for any reason (Ready transitions to false, is evicted, or is drained) an updated pod is immediatedly created on that node without considering surge limits. Allowing surge implies the possibility that the resources consumed by the daemonset on any given node can double if the readiness check fails, and so resource intensive daemonsets should take into account that they may cause evictions during disruption.
"%
       | optional,
    maxUnavailable
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
The maximum number of DaemonSet pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0 if MaxSurge is 0 Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.
"%
       | optional
  },

  RollingUpdateDeployment = {
    maxSurge
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
The maximum number of pods that can be scheduled above the desired number of pods. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up. Defaults to 25%. Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when the rolling update starts, such that the total number of old and new pods do not exceed 130% of desired pods. Once old pods have been killed, new ReplicaSet can be scaled up further, ensuring that total number of pods running at any time during the update is at most 130% of desired pods.
"%
       | optional,
    maxUnavailable
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). Absolute number is calculated from percentage by rounding down. This can not be 0 if MaxSurge is 0. Defaults to 25%. Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods immediately when the rolling update starts. Once new pods are ready, old ReplicaSet can be scaled down further, followed by scaling up the new ReplicaSet, ensuring that the total number of pods available at all times during the update is at least 70% of desired pods.
"%
       | optional
  },

  RollingUpdateStatefulSetStrategy = {
    maxUnavailable
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). Absolute number is calculated from percentage by rounding up. This can not be 0. Defaults to 1. This field is alpha-level and is only honored by servers that enable the MaxUnavailableStatefulSet feature. The field applies to all pods in the range 0 to Replicas-1. That means if there is any unavailable pod in the range 0 to Replicas-1, it will be counted towards MaxUnavailable.
"%
       | optional,
    partition
       | Number
       | doc m%"
Partition indicates the ordinal at which the StatefulSet should be partitioned for updates. During a rolling update, all pods from ordinal Replicas-1 to Partition are updated. All pods from ordinal Partition-1 to 0 remain untouched. This is helpful in being able to do a canary based deployment. The default value is 0.
"%
       | optional
  },

  StatefulSet = {
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
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.apps.v1.StatefulSetSpec
       | doc "Spec defines the desired identities of pods in this set."
       | optional,
    status
       | io.k8s.api.apps.v1.StatefulSetStatus
       | doc m%"
Status is the current status of Pods in this StatefulSet. This data may be out of date by some window of time.
"%
       | optional
  },

  StatefulSetCondition = {
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "A human readable message indicating details about the transition."
       | optional,
    reason
       | String
       | doc "The reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of statefulset condition."
  },

  StatefulSetList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apps.v1.StatefulSet
       | doc "Items is the list of stateful sets.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc m%"
Standard list's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  StatefulSetOrdinals = {
    start
       | Number
       | doc m%"
start is the number representing the first replica's index. It may be used to number replicas from an alternate index (eg: 1-indexed) over the default 0-indexed names, or to orchestrate progressive movement of replicas from one StatefulSet to another. If set, replica indices will be in the range:
  [.spec.ordinals.start, .spec.ordinals.start + .spec.replicas).
If unset, defaults to 0. Replica indices will be in the range:
  [0, .spec.replicas).
"%
       | optional
  },

  StatefulSetPersistentVolumeClaimRetentionPolicy = {
    whenDeleted
       | String
       | doc m%"
WhenDeleted specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is deleted. The default policy of `Retain` causes PVCs to not be affected by StatefulSet deletion. The `Delete` policy causes those PVCs to be deleted.
"%
       | optional,
    whenScaled
       | String
       | doc m%"
WhenScaled specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is scaled down. The default policy of `Retain` causes PVCs to not be affected by a scaledown. The `Delete` policy causes the associated PVCs for any excess pods above the replica count to be deleted.
"%
       | optional
  },

  StatefulSetSpec = {
    minReadySeconds
       | Number
       | doc m%"
Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
"%
       | optional,
    ordinals
       | io.k8s.api.apps.v1.StatefulSetOrdinals
       | doc m%"
ordinals controls the numbering of replica indices in a StatefulSet. The default ordinals behavior assigns a "0" index to the first replica and increments the index by one for each additional replica requested.
"%
       | optional,
    persistentVolumeClaimRetentionPolicy
       | io.k8s.api.apps.v1.StatefulSetPersistentVolumeClaimRetentionPolicy
       | doc m%"
persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down.
"%
       | optional,
    podManagementPolicy
       | String
       | doc m%"
podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
"%
       | optional,
    replicas
       | Number
       | doc m%"
replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
"%
       | optional,
    revisionHistoryLimit
       | Number
       | doc m%"
revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
"%
       | optional,
    selector
       | LabelSelector
       | doc m%"
selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
"%,
    serviceName
       | String
       | doc m%"
serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where "pod-specific-string" is managed by the StatefulSet controller.
"%
       | optional,
    template
       | PodTemplateSpec
       | doc m%"
template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet. Each pod will be named with the format <statefulsetname>-<podindex>. For example, a pod in a StatefulSet named "web" with index number "3" would be named "web-3". The only allowed template.spec.restartPolicy value is "Always".
"%,
    updateStrategy
       | io.k8s.api.apps.v1.StatefulSetUpdateStrategy
       | doc m%"
updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.
"%
       | optional,
    volumeClaimTemplates
       | Array PersistentVolumeClaim
       | doc m%"
volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
"%
       | optional
  },

  StatefulSetStatus = {
    availableReplicas
       | Number
       | doc m%"
Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
"%
       | optional,
    collisionCount
       | Number
       | doc m%"
collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
"%
       | optional,
    conditions
       | Array io.k8s.api.apps.v1.StatefulSetCondition
       | doc "Represents the latest available observations of a statefulset's current state."
       | optional,
    currentReplicas
       | Number
       | doc m%"
currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
"%
       | optional,
    currentRevision
       | String
       | doc m%"
currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).
"%
       | optional,
    observedGeneration
       | Number
       | doc m%"
observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
"%
       | optional,
    readyReplicas
       | Number
       | doc m%"
readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
"%
       | optional,
    replicas
       | Number
       | doc "replicas is the number of Pods created by the StatefulSet controller.",
    updateRevision
       | String
       | doc m%"
updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
"%
       | optional,
    updatedReplicas
       | Number
       | doc m%"
updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
"%
       | optional
  },

  StatefulSetUpdateStrategy = {
    rollingUpdate
       | io.k8s.api.apps.v1.RollingUpdateStatefulSetStrategy
       | doc m%"
RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
"%
       | optional,
    type
       | String
       | doc m%"
Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
"%
       | optional
  }
}