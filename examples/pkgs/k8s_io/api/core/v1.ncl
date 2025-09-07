{
  AWSElasticBlockStoreVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
       | optional,
    partition
       | Number
       | doc m%"
partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly value true will force the readOnly setting in VolumeMounts. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
       | optional,
    volumeID
       | String
       | doc m%"
volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
  },

  Affinity = {
    nodeAffinity
       | NodeAffinity
       | doc "Describes node affinity scheduling rules for the pod."
       | optional,
    podAffinity
       | PodAffinity
       | doc m%"
Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
"%
       | optional,
    podAntiAffinity
       | PodAntiAffinity
       | doc m%"
Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
"%
       | optional
  },

  AppArmorProfile = {
    localhostProfile
       | String
       | doc m%"
localhostProfile indicates a profile loaded on the node that should be used. The profile must be preconfigured on the node to work. Must match the loaded name of the profile. Must be set if and only if type is "Localhost".
"%
       | optional,
    type
       | String
       | doc m%"
type indicates which kind of AppArmor profile will be applied. Valid options are:
  Localhost - a profile pre-loaded on the node.
  RuntimeDefault - the container runtime's default profile.
  Unconfined - no AppArmor enforcement.
"%
  },

  AttachedVolume = {
    devicePath
       | String
       | doc "DevicePath represents the device path where the volume should be available",
    name
       | String
       | doc "Name of the attached volume"
  },

  AzureDiskVolumeSource = {
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
fsType is Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    kind
       | String
       | doc m%"
kind expected values are Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional
  },

  AzureFilePersistentVolumeSource = {
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretName
       | String
       | doc m%"
secretName is the name of secret that contains Azure Storage Account Name and Key
"%,
    secretNamespace
       | String
       | doc m%"
secretNamespace is the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod
"%
       | optional,
    shareName
       | String
       | doc "shareName is the azure Share Name"
  },

  AzureFileVolumeSource = {
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
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
  },

  Binding = {
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
    target
       | ObjectReference
       | doc "The target object that you want to bind to the standard object."
  },

  CSIPersistentVolumeSource = {
    controllerExpandSecretRef
       | SecretReference
       | doc m%"
controllerExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerExpandVolume call. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
"%
       | optional,
    controllerPublishSecretRef
       | SecretReference
       | doc m%"
controllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
"%
       | optional,
    driver
       | String
       | doc "driver is the name of the driver to use for this volume. Required.",
    fsType
       | String
       | doc m%"
fsType to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs".
"%
       | optional,
    nodeExpandSecretRef
       | SecretReference
       | doc m%"
nodeExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeExpandVolume call. This field is optional, may be omitted if no secret is required. If the secret object contains more than one secret, all secrets are passed.
"%
       | optional,
    nodePublishSecretRef
       | SecretReference
       | doc m%"
nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
"%
       | optional,
    nodeStageSecretRef
       | SecretReference
       | doc m%"
nodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
"%
       | optional,
    volumeAttributes
       | { .. }
       | doc "volumeAttributes of the volume to publish."
       | optional,
    volumeHandle
       | String
       | doc m%"
volumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
"%
  },

  CSIVolumeSource = {
    driver
       | String
       | doc m%"
driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
"%,
    fsType
       | String
       | doc m%"
fsType to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
"%
       | optional,
    nodePublishSecretRef
       | LocalObjectReference
       | doc m%"
nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).
"%
       | optional,
    volumeAttributes
       | { .. }
       | doc m%"
volumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
"%
       | optional
  },

  Capabilities = {
    add
       | Array String
       | doc "Added capabilities"
       | optional,
    drop
       | Array String
       | doc "Removed capabilities"
       | optional
  },

  CephFSPersistentVolumeSource = {
    monitors
       | Array String
       | doc m%"
monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
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
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    secretFile
       | String
       | doc m%"
secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    secretRef
       | SecretReference
       | doc m%"
secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    user
       | String
       | doc m%"
user is Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional
  },

  CephFSVolumeSource = {
    monitors
       | Array String
       | doc m%"
monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
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
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    secretFile
       | String
       | doc m%"
secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional,
    user
       | String
       | doc m%"
user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
"%
       | optional
  },

  CinderPersistentVolumeSource = {
    fsType
       | String
       | doc m%"
fsType Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    secretRef
       | SecretReference
       | doc m%"
secretRef is Optional: points to a secret object containing parameters used to connect to OpenStack.
"%
       | optional,
    volumeID
       | String
       | doc m%"
volumeID used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
  },

  CinderVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef is optional: points to a secret object containing parameters used to connect to OpenStack.
"%
       | optional,
    volumeID
       | String
       | doc m%"
volumeID used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
  },

  ClientIPConfig = {
    timeoutSeconds
       | Number
       | doc m%"
timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
"%
       | optional
  },

  ClusterTrustBundleProjection = {
    labelSelector
       | LabelSelector
       | doc m%"
Select all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as "match nothing".  If set but empty, interpreted as "match everything".
"%
       | optional,
    name
       | String
       | doc m%"
Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.
"%
       | optional,
    "optional"
       | Bool
       | doc m%"
If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.
"%
       | optional,
    path
       | String
       | doc "Relative path from the volume root to write the bundle.",
    signerName
       | String
       | doc m%"
Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.
"%
       | optional
  },

  ComponentCondition = {
    error
       | String
       | doc "Condition error code for a component. For example, a health check error code."
       | optional,
    message
       | String
       | doc m%"
Message about the condition for a component. For example, information about a health check.
"%
       | optional,
    status
       | String
       | doc m%"
Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
"%,
    type
       | String
       | doc "Type of condition for a component. Valid value: \"Healthy\""
  },

  ComponentStatus = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    conditions
       | Array ComponentCondition
       | doc "List of component conditions observed"
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
       | optional
  },

  ComponentStatusList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ComponentStatus
       | doc "List of ComponentStatus objects.",
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

  ConfigMap = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    binaryData
       | { .. }
       | doc m%"
BinaryData contains the binary data. Each key must consist of alphanumeric characters, '-', '_' or '.'. BinaryData can contain byte sequences that are not in the UTF-8 range. The keys stored in BinaryData must not overlap with the ones in the Data field, this is enforced during validation process. Using this field will require 1.10+ apiserver and kubelet.
"%
       | optional,
    data
       | { .. }
       | doc m%"
Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'. Values with non-UTF-8 byte sequences must use the BinaryData field. The keys stored in Data must not overlap with the keys in the BinaryData field, this is enforced during validation process.
"%
       | optional,
    immutable
       | Bool
       | doc m%"
Immutable, if set to true, ensures that data stored in the ConfigMap cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
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
       | optional
  },

  ConfigMapEnvSource = {
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "Specify whether the ConfigMap must be defined"
       | optional
  },

  ConfigMapKeySelector = {
    key
       | String
       | doc "The key to select.",
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "Specify whether the ConfigMap or its key must be defined"
       | optional
  },

  ConfigMapList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ConfigMap
       | doc "Items is the list of ConfigMaps.",
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

  ConfigMapNodeConfigSource = {
    kubeletConfigKey
       | String
       | doc m%"
KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
"%,
    name
       | String
       | doc m%"
Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
"%,
    namespace
       | String
       | doc m%"
Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
"%,
    resourceVersion
       | String
       | doc m%"
ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
"%
       | optional,
    uid
       | String
       | doc m%"
UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
"%
       | optional
  },

  ConfigMapProjection = {
    items
       | Array KeyToPath
       | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
"%
       | optional,
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "optional specify whether the ConfigMap or its keys must be defined"
       | optional
  },

  ConfigMapVolumeSource = {
    defaultMode
       | Number
       | doc m%"
defaultMode is optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    items
       | Array KeyToPath
       | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
"%
       | optional,
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "optional specify whether the ConfigMap or its keys must be defined"
       | optional
  },

  Container = {
    args
       | Array String
       | doc m%"
Arguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
       | optional,
    command
       | Array String
       | doc m%"
Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
       | optional,
    env
       | Array EnvVar
       | doc "List of environment variables to set in the container. Cannot be updated."
       | optional,
    envFrom
       | Array EnvFromSource
       | doc m%"
List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
"%
       | optional,
    image
       | String
       | doc m%"
Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
"%
       | optional,
    imagePullPolicy
       | String
       | doc m%"
Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%
       | optional,
    lifecycle
       | Lifecycle
       | doc m%"
Actions that the management system should take in response to container lifecycle events. Cannot be updated.
"%
       | optional,
    livenessProbe
       | Probe
       | doc m%"
Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
       | optional,
    name
       | String
       | doc m%"
Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
"%,
    ports
       | Array ContainerPort
       | doc m%"
List of ports to expose from the container. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Modifying this array with strategic merge patch may corrupt the data. For more information See https://github.com/kubernetes/kubernetes/issues/108255. Cannot be updated.
"%
       | optional,
    readinessProbe
       | Probe
       | doc m%"
Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
       | optional,
    resizePolicy
       | Array ContainerResizePolicy
       | doc "Resources resize policy for the container."
       | optional,
    resources
       | ResourceRequirements
       | doc m%"
Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
       | optional,
    restartPolicy
       | String
       | doc m%"
RestartPolicy defines the restart behavior of individual containers in a pod. This field may only be set for init containers, and the only allowed value is "Always". For non-init containers or when this field is not specified, the restart behavior is defined by the Pod's restart policy and the container type. Setting the RestartPolicy as "Always" for the init container will have the following effect: this init container will be continually restarted on exit until all regular containers have terminated. Once all regular containers have completed, all init containers with restartPolicy "Always" will be shut down. This lifecycle differs from normal init containers and is often referred to as a "sidecar" container. Although this init container still starts in the init container sequence, it does not wait for the container to complete before proceeding to the next init container. Instead, the next init container starts immediately after this init container is started, or after any startupProbe has successfully completed.
"%
       | optional,
    securityContext
       | SecurityContext
       | doc m%"
SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
"%
       | optional,
    startupProbe
       | Probe
       | doc m%"
StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
       | optional,
    stdin
       | Bool
       | doc m%"
Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
"%
       | optional,
    stdinOnce
       | Bool
       | doc m%"
Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
"%
       | optional,
    terminationMessagePath
       | String
       | doc m%"
Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
"%
       | optional,
    terminationMessagePolicy
       | String
       | doc m%"
Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
"%
       | optional,
    tty
       | Bool
       | doc m%"
Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
"%
       | optional,
    volumeDevices
       | Array VolumeDevice
       | doc "volumeDevices is the list of block devices to be used by the container."
       | optional,
    volumeMounts
       | Array VolumeMount
       | doc "Pod volumes to mount into the container's filesystem. Cannot be updated."
       | optional,
    workingDir
       | String
       | doc m%"
Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
"%
       | optional
  },

  ContainerImage = {
    names
       | Array String
       | doc m%"
Names by which this image is known. e.g. ["kubernetes.example/hyperkube:v1.0.7", "cloud-vendor.registry.example/cloud-vendor/hyperkube:v1.0.7"]
"%
       | optional,
    sizeBytes
       | Number
       | doc "The size of the image in bytes."
       | optional
  },

  ContainerPort = {
    containerPort
       | Number
       | doc m%"
Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
"%,
    hostIP
       | String
       | doc "What host IP to bind the external port to."
       | optional,
    hostPort
       | Number
       | doc m%"
Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
"%
       | optional,
    name
       | String
       | doc m%"
If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
"%
       | optional,
    protocol
       | String
       | doc "Protocol for port. Must be UDP, TCP, or SCTP. Defaults to \"TCP\"."
       | optional
  },

  ContainerResizePolicy = {
    resourceName
       | String
       | doc m%"
Name of the resource to which this resource resize policy applies. Supported values: cpu, memory.
"%,
    restartPolicy
       | String
       | doc m%"
Restart policy to apply when specified resource is resized. If not specified, it defaults to NotRequired.
"%
  },

  ContainerState = {
    running
       | ContainerStateRunning
       | doc "Details about a running container"
       | optional,
    terminated
       | ContainerStateTerminated
       | doc "Details about a terminated container"
       | optional,
    waiting
       | ContainerStateWaiting
       | doc "Details about a waiting container"
       | optional
  },

  ContainerStateRunning = {
    startedAt
       | Time
       | doc "Time at which the container was last (re-)started"
       | optional
  },

  ContainerStateTerminated = {
    containerID
       | String
       | doc "Container's ID in the format '<type>://<container_id>'"
       | optional,
    exitCode
       | Number
       | doc "Exit status from the last termination of the container",
    finishedAt
       | Time
       | doc "Time at which the container last terminated"
       | optional,
    message
       | String
       | doc "Message regarding the last termination of the container"
       | optional,
    reason
       | String
       | doc "(brief) reason from the last termination of the container"
       | optional,
    signal
       | Number
       | doc "Signal from the last termination of the container"
       | optional,
    startedAt
       | Time
       | doc "Time at which previous execution of the container started"
       | optional
  },

  ContainerStateWaiting = {
    message
       | String
       | doc "Message regarding why the container is not yet running."
       | optional,
    reason
       | String
       | doc "(brief) reason the container is not yet running."
       | optional
  },

  ContainerStatus = {
    allocatedResources
       | { .. }
       | doc m%"
AllocatedResources represents the compute resources allocated for this container by the node. Kubelet sets this value to Container.Resources.Requests upon successful pod admission and after successfully admitting desired pod resize.
"%
       | optional,
    allocatedResourcesStatus
       | Array ResourceStatus
       | doc m%"
AllocatedResourcesStatus represents the status of various resources allocated for this Pod.
"%
       | optional,
    containerID
       | String
       | doc m%"
ContainerID is the ID of the container in the format '<type>://<container_id>'. Where type is a container runtime identifier, returned from Version call of CRI API (for example "containerd").
"%
       | optional,
    image
       | String
       | doc m%"
Image is the name of container image that the container is running. The container image may not match the image used in the PodSpec, as it may have been resolved by the runtime. More info: https://kubernetes.io/docs/concepts/containers/images.
"%,
    imageID
       | String
       | doc m%"
ImageID is the image ID of the container's image. The image ID may not match the image ID of the image used in the PodSpec, as it may have been resolved by the runtime.
"%,
    lastState
       | ContainerState
       | doc m%"
LastTerminationState holds the last termination state of the container to help debug container crashes and restarts. This field is not populated if the container is still running and RestartCount is 0.
"%
       | optional,
    name
       | String
       | doc m%"
Name is a DNS_LABEL representing the unique name of the container. Each container in a pod must have a unique name across all container types. Cannot be updated.
"%,
    ready
       | Bool
       | doc m%"
Ready specifies whether the container is currently passing its readiness check. The value will change as readiness probes keep executing. If no readiness probes are specified, this field defaults to true once the container is fully started (see Started field).

The value is typically used to determine whether a container is ready to accept traffic.
"%,
    resources
       | ResourceRequirements
       | doc m%"
Resources represents the compute resource requests and limits that have been successfully enacted on the running container after it has been started or has been successfully resized.
"%
       | optional,
    restartCount
       | Number
       | doc m%"
RestartCount holds the number of times the container has been restarted. Kubelet makes an effort to always increment the value, but there are cases when the state may be lost due to node restarts and then the value may be reset to 0. The value is never negative.
"%,
    started
       | Bool
       | doc m%"
Started indicates whether the container has finished its postStart lifecycle hook and passed its startup probe. Initialized as false, becomes true after startupProbe is considered successful. Resets to false when the container is restarted, or if kubelet loses state temporarily. In both cases, startup probes will run again. Is always true when no startupProbe is defined and container is running and has passed the postStart lifecycle hook. The null value must be treated the same as false.
"%
       | optional,
    state
       | ContainerState
       | doc "State holds details about the container's current condition."
       | optional,
    stopSignal
       | String
       | doc "StopSignal reports the effective stop signal for this container"
       | optional,
    user
       | ContainerUser
       | doc m%"
User represents user identity information initially attached to the first process of the container
"%
       | optional,
    volumeMounts
       | Array VolumeMountStatus
       | doc "Status of volume mounts."
       | optional
  },

  ContainerUser = {
    linux
       | LinuxContainerUser
       | doc m%"
Linux holds user identity information initially attached to the first process of the containers in Linux. Note that the actual running identity can be changed if the process has enough privilege to do so.
"%
       | optional
  },

  DaemonEndpoint = {
    Port
       | Number
       | doc "Port number of the given endpoint."
  },

  DownwardAPIProjection = {
    items
       | Array DownwardAPIVolumeFile
       | doc "Items is a list of DownwardAPIVolume file"
       | optional
  },

  DownwardAPIVolumeFile = {
    fieldRef
       | ObjectFieldSelector
       | doc m%"
Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
"%
       | optional,
    mode
       | Number
       | doc m%"
Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    path
       | String
       | doc m%"
Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
"%,
    resourceFieldRef
       | ResourceFieldSelector
       | doc m%"
Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
"%
       | optional
  },

  DownwardAPIVolumeSource = {
    defaultMode
       | Number
       | doc m%"
Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    items
       | Array DownwardAPIVolumeFile
       | doc "Items is a list of downward API volume file"
       | optional
  },

  EmptyDirVolumeSource = {
    medium
       | String
       | doc m%"
medium represents what type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
       | optional,
    sizeLimit
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc m%"
sizeLimit is the total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
       | optional
  },

  EndpointAddress = {
    hostname
       | String
       | doc "The Hostname of this endpoint"
       | optional,
    ip
       | String
       | doc m%"
The IP of this endpoint. May not be loopback (127.0.0.0/8 or ::1), link-local (169.254.0.0/16 or fe80::/10), or link-local multicast (224.0.0.0/24 or ff02::/16).
"%,
    nodeName
       | String
       | doc m%"
Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
"%
       | optional,
    targetRef
       | ObjectReference
       | doc "Reference to object providing the endpoint."
       | optional
  },

  EndpointPort = {
    appProtocol
       | String
       | doc m%"
The application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:

* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).

* Kubernetes-defined prefixed names:
  * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
  * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
  * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455

* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.
"%
       | optional,
    name
       | String
       | doc m%"
The name of this port.  This must match the 'name' field in the corresponding ServicePort. Must be a DNS_LABEL. Optional only if one port is defined.
"%
       | optional,
    port
       | Number
       | doc "The port number of the endpoint.",
    protocol
       | String
       | doc "The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP."
       | optional
  },

  EndpointSubset = {
    addresses
       | Array EndpointAddress
       | doc m%"
IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
"%
       | optional,
    notReadyAddresses
       | Array EndpointAddress
       | doc m%"
IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.
"%
       | optional,
    ports
       | Array EndpointPort
       | doc "Port numbers available on the related IP addresses."
       | optional
  },

  Endpoints = {
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
    subsets
       | Array EndpointSubset
       | doc m%"
The set of all endpoints is the union of all subsets. Addresses are placed into subsets according to the IPs they share. A single address with multiple ports, some of which are ready and some of which are not (because they come from different containers) will result in the address being displayed in different subsets for the different ports. No address will appear in both Addresses and NotReadyAddresses in the same subset. Sets of addresses and ports that comprise a service.
"%
       | optional
  },

  EndpointsList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Endpoints
       | doc "List of endpoints.",
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

  EnvFromSource = {
    configMapRef
       | ConfigMapEnvSource
       | doc "The ConfigMap to select from"
       | optional,
    prefix
       | String
       | doc m%"
Optional text to prepend to the name of each environment variable. Must be a C_IDENTIFIER.
"%
       | optional,
    secretRef
       | SecretEnvSource
       | doc "The Secret to select from"
       | optional
  },

  EnvVar = {
    name
       | String
       | doc "Name of the environment variable. Must be a C_IDENTIFIER.",
    value
       | String
       | doc m%"
Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
"%
       | optional,
    valueFrom
       | EnvVarSource
       | doc m%"
Source for the environment variable's value. Cannot be used if value is not empty.
"%
       | optional
  },

  EnvVarSource = {
    configMapKeyRef
       | ConfigMapKeySelector
       | doc "Selects a key of a ConfigMap."
       | optional,
    fieldRef
       | ObjectFieldSelector
       | doc m%"
Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
"%
       | optional,
    resourceFieldRef
       | ResourceFieldSelector
       | doc m%"
Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
"%
       | optional,
    secretKeyRef
       | SecretKeySelector
       | doc "Selects a key of a secret in the pod's namespace"
       | optional
  },

  EphemeralContainer = {
    args
       | Array String
       | doc m%"
Arguments to the entrypoint. The image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
       | optional,
    command
       | Array String
       | doc m%"
Entrypoint array. Not executed within a shell. The image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
"%
       | optional,
    env
       | Array EnvVar
       | doc "List of environment variables to set in the container. Cannot be updated."
       | optional,
    envFrom
       | Array EnvFromSource
       | doc m%"
List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
"%
       | optional,
    image
       | String
       | doc m%"
Container image name. More info: https://kubernetes.io/docs/concepts/containers/images
"%
       | optional,
    imagePullPolicy
       | String
       | doc m%"
Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
"%
       | optional,
    lifecycle
       | Lifecycle
       | doc "Lifecycle is not allowed for ephemeral containers."
       | optional,
    livenessProbe
       | Probe
       | doc "Probes are not allowed for ephemeral containers."
       | optional,
    name
       | String
       | doc m%"
Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.
"%,
    ports
       | Array ContainerPort
       | doc "Ports are not allowed for ephemeral containers."
       | optional,
    readinessProbe
       | Probe
       | doc "Probes are not allowed for ephemeral containers."
       | optional,
    resizePolicy
       | Array ContainerResizePolicy
       | doc "Resources resize policy for the container."
       | optional,
    resources
       | ResourceRequirements
       | doc m%"
Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
"%
       | optional,
    restartPolicy
       | String
       | doc m%"
Restart policy for the container to manage the restart behavior of each container within a pod. This may only be set for init containers. You cannot set this field on ephemeral containers.
"%
       | optional,
    securityContext
       | SecurityContext
       | doc m%"
Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
"%
       | optional,
    startupProbe
       | Probe
       | doc "Probes are not allowed for ephemeral containers."
       | optional,
    stdin
       | Bool
       | doc m%"
Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
"%
       | optional,
    stdinOnce
       | Bool
       | doc m%"
Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
"%
       | optional,
    targetContainerName
       | String
       | doc m%"
If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec.

The container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.
"%
       | optional,
    terminationMessagePath
       | String
       | doc m%"
Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
"%
       | optional,
    terminationMessagePolicy
       | String
       | doc m%"
Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
"%
       | optional,
    tty
       | Bool
       | doc m%"
Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
"%
       | optional,
    volumeDevices
       | Array VolumeDevice
       | doc "volumeDevices is the list of block devices to be used by the container."
       | optional,
    volumeMounts
       | Array VolumeMount
       | doc m%"
Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.
"%
       | optional,
    workingDir
       | String
       | doc m%"
Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
"%
       | optional
  },

  EphemeralVolumeSource = {
    volumeClaimTemplate
       | PersistentVolumeClaimTemplate
       | doc m%"
Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).

An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.

This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.

Required, must not be nil.
"%
       | optional
  },

  Event = {
    action
       | String
       | doc "What action was taken/failed regarding to the Regarding object."
       | optional,
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    count
       | Number
       | doc "The number of times this event has occurred."
       | optional,
    eventTime
       | MicroTime
       | doc "Time when this Event was first observed."
       | optional,
    firstTimestamp
       | Time
       | doc m%"
The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
"%
       | optional,
    involvedObject
       | ObjectReference
       | doc "The object that this event is about.",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    lastTimestamp
       | Time
       | doc "The time at which the most recent occurrence of this event was recorded."
       | optional,
    message
       | String
       | doc "A human-readable description of the status of this operation."
       | optional,
    metadata
       | ObjectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
    reason
       | String
       | doc m%"
This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
"%
       | optional,
    related
       | ObjectReference
       | doc "Optional secondary object for more complex actions."
       | optional,
    reportingComponent
       | String
       | doc "Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`."
       | optional,
    reportingInstance
       | String
       | doc "ID of the controller instance, e.g. `kubelet-xyzf`."
       | optional,
    series
       | EventSeries
       | doc m%"
Data about the Event series this event represents or nil if it's a singleton Event.
"%
       | optional,
    source
       | EventSource
       | doc m%"
The component reporting this event. Should be a short machine understandable string.
"%
       | optional,
    type
       | String
       | doc "Type of this event (Normal, Warning), new types could be added in the future"
       | optional
  },

  EventList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Event
       | doc "List of events",
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

  EventSeries = {
    count
       | Number
       | doc "Number of occurrences in this series up to the last heartbeat time"
       | optional,
    lastObservedTime
       | MicroTime
       | doc "Time of the last occurrence observed"
       | optional
  },

  EventSource = {
    component
       | String
       | doc "Component from which the event is generated."
       | optional,
    host
       | String
       | doc "Node name on which the event is generated."
       | optional
  },

  ExecAction = {
    command
       | Array String
       | doc m%"
Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
"%
       | optional
  },

  FCVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    lun
       | Number
       | doc "lun is Optional: FC target lun number"
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    targetWWNs
       | Array String
       | doc "targetWWNs is Optional: FC target worldwide names (WWNs)"
       | optional,
    wwids
       | Array String
       | doc m%"
wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
"%
       | optional
  },

  FlexPersistentVolumeSource = {
    driver
       | String
       | doc "driver is the name of the driver to use for this volume.",
    fsType
       | String
       | doc m%"
fsType is the Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%
       | optional,
    options
       | { .. }
       | doc "options is Optional: this field holds extra command options if any."
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | SecretReference
       | doc m%"
secretRef is Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
"%
       | optional
  },

  FlexVolumeSource = {
    driver
       | String
       | doc "driver is the name of the driver to use for this volume.",
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
"%
       | optional,
    options
       | { .. }
       | doc "options is Optional: this field holds extra command options if any."
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef is Optional: secretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
"%
       | optional
  },

  FlockerVolumeSource = {
    datasetName
       | String
       | doc m%"
datasetName is Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
"%
       | optional,
    datasetUUID
       | String
       | doc m%"
datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
"%
       | optional
  },

  GCEPersistentDiskVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
       | optional,
    partition
       | Number
       | doc m%"
partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
       | optional,
    pdName
       | String
       | doc m%"
pdName is unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
       | optional
  },

  GRPCAction = {
    port
       | Number
       | doc "Port number of the gRPC service. Number must be in the range 1 to 65535.",
    service
       | String
       | doc m%"
Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).

If this is not specified, the default behavior is defined by gRPC.
"%
       | optional
  },

  GitRepoVolumeSource = {
    directory
       | String
       | doc m%"
directory is the target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
"%
       | optional,
    repository
       | String
       | doc "repository is the URL",
    revision
       | String
       | doc "revision is the commit hash for the specified revision."
       | optional
  },

  GlusterfsPersistentVolumeSource = {
    endpoints
       | String
       | doc m%"
endpoints is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
    endpointsNamespace
       | String
       | doc m%"
endpointsNamespace is the namespace that contains Glusterfs endpoint. If this field is empty, the EndpointNamespace defaults to the same namespace as the bound PVC. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%
       | optional,
    path
       | String
       | doc m%"
path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%
       | optional
  },

  GlusterfsVolumeSource = {
    endpoints
       | String
       | doc m%"
endpoints is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
    path
       | String
       | doc m%"
path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
"%
       | optional
  },

  HTTPGetAction = {
    host
       | String
       | doc m%"
Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
"%
       | optional,
    httpHeaders
       | Array HTTPHeader
       | doc "Custom headers to set in the request. HTTP allows repeated headers."
       | optional,
    path
       | String
       | doc "Path to access on the HTTP server."
       | optional,
    port
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
"%,
    scheme
       | String
       | doc "Scheme to use for connecting to the host. Defaults to HTTP."
       | optional
  },

  HTTPHeader = {
    name
       | String
       | doc m%"
The header field name. This will be canonicalized upon output, so case-variant names will be understood as the same header.
"%,
    value
       | String
       | doc "The header field value"
  },

  HostAlias = {
    hostnames
       | Array String
       | doc "Hostnames for the above IP address."
       | optional,
    ip
       | String
       | doc "IP address of the host file entry."
  },

  HostIP = {
    ip
       | String
       | doc "IP is the IP address assigned to the host"
  },

  HostPathVolumeSource = {
    path
       | String
       | doc m%"
path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%,
    type
       | String
       | doc m%"
type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
       | optional
  },

  ISCSIPersistentVolumeSource = {
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
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
"%
       | optional,
    initiatorName
       | String
       | doc m%"
initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
"%
       | optional,
    iqn
       | String
       | doc "iqn is Target iSCSI Qualified Name.",
    iscsiInterface
       | String
       | doc m%"
iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
"%
       | optional,
    lun
       | Number
       | doc "lun is iSCSI Target Lun number.",
    portals
       | Array String
       | doc m%"
portals is the iSCSI Target Portal List. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
"%
       | optional,
    secretRef
       | SecretReference
       | doc "secretRef is the CHAP Secret for iSCSI target and initiator authentication"
       | optional,
    targetPortal
       | String
       | doc m%"
targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%
  },

  ISCSIVolumeSource = {
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
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
"%
       | optional,
    initiatorName
       | String
       | doc m%"
initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
"%
       | optional,
    iqn
       | String
       | doc "iqn is the target iSCSI Qualified Name.",
    iscsiInterface
       | String
       | doc m%"
iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
"%
       | optional,
    lun
       | Number
       | doc "lun represents iSCSI Target Lun number.",
    portals
       | Array String
       | doc m%"
portals is the iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc "secretRef is the CHAP Secret for iSCSI target and initiator authentication"
       | optional,
    targetPortal
       | String
       | doc m%"
targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
"%
  },

  ImageVolumeSource = {
    pullPolicy
       | String
       | doc m%"
Policy for pulling OCI objects. Possible values are: Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
"%
       | optional,
    reference
       | String
       | doc m%"
Required: Image or artifact reference to be used. Behaves in the same way as pod.spec.containers[*].image. Pull secrets will be assembled in the same way as for the container image by looking up node credentials, SA image pull secrets, and pod spec image pull secrets. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
"%
       | optional
  },

  KeyToPath = {
    key
       | String
       | doc "key is the key to project.",
    mode
       | Number
       | doc m%"
mode is Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    path
       | String
       | doc m%"
path is the relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
"%
  },

  Lifecycle = {
    postStart
       | LifecycleHandler
       | doc m%"
PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
       | optional,
    preStop
       | LifecycleHandler
       | doc m%"
PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
"%
       | optional,
    stopSignal
       | String
       | doc m%"
StopSignal defines which signal will be sent to a container when it is being stopped. If not specified, the default is defined by the container runtime in use. StopSignal can only be set for Pods with a non-empty .spec.os.name
"%
       | optional
  },

  LifecycleHandler = {
    exec
       | ExecAction
       | doc "Exec specifies a command to execute in the container."
       | optional,
    httpGet
       | HTTPGetAction
       | doc "HTTPGet specifies an HTTP GET request to perform."
       | optional,
    sleep
       | SleepAction
       | doc "Sleep represents a duration that the container should sleep."
       | optional,
    tcpSocket
       | TCPSocketAction
       | doc m%"
Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for backward compatibility. There is no validation of this field and lifecycle hooks will fail at runtime when it is specified.
"%
       | optional
  },

  LimitRange = {
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
       | LimitRangeSpec
       | doc m%"
Spec defines the limits enforced. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  LimitRangeItem = {
    "default"
       | { .. }
       | doc m%"
Default resource requirement limit value by resource name if resource limit is omitted.
"%
       | optional,
    defaultRequest
       | { .. }
       | doc m%"
DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
"%
       | optional,
    max
       | { .. }
       | doc "Max usage constraints on this kind by resource name."
       | optional,
    maxLimitRequestRatio
       | { .. }
       | doc m%"
MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
"%
       | optional,
    min
       | { .. }
       | doc "Min usage constraints on this kind by resource name."
       | optional,
    type
       | String
       | doc "Type of resource that this limit applies to."
  },

  LimitRangeList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array LimitRange
       | doc m%"
Items is a list of LimitRange objects. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
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

  LimitRangeSpec = {
    limits
       | Array LimitRangeItem
       | doc "Limits is the list of LimitRangeItem objects that are enforced."
  },

  LinuxContainerUser = {
    gid
       | Number
       | doc "GID is the primary gid initially attached to the first process in the container",
    supplementalGroups
       | Array Number
       | doc m%"
SupplementalGroups are the supplemental groups initially attached to the first process in the container
"%
       | optional,
    uid
       | Number
       | doc "UID is the primary uid initially attached to the first process in the container"
  },

  LoadBalancerIngress = {
    hostname
       | String
       | doc m%"
Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
"%
       | optional,
    ip
       | String
       | doc m%"
IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
"%
       | optional,
    ipMode
       | String
       | doc m%"
IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to "VIP" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to "Proxy" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing.
"%
       | optional,
    ports
       | Array PortStatus
       | doc m%"
Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
"%
       | optional
  },

  LoadBalancerStatus = {
    ingress
       | Array LoadBalancerIngress
       | doc m%"
Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
"%
       | optional
  },

  LocalObjectReference = {
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional
  },

  LocalVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a filesystem if unspecified.
"%
       | optional,
    path
       | String
       | doc m%"
path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
"%
  },

  ModifyVolumeStatus = {
    status
       | String
       | doc m%"
status is the status of the ControllerModifyVolume operation. It can be in any of following states:
 - Pending
   Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as
   the specified VolumeAttributesClass not existing.
 - InProgress
   InProgress indicates that the volume is being modified.
 - Infeasible
  Infeasible indicates that the request has been rejected as invalid by the CSI driver. To
	  resolve the error, a valid VolumeAttributesClass needs to be specified.
Note: New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately.
"%,
    targetVolumeAttributesClassName
       | String
       | doc m%"
targetVolumeAttributesClassName is the name of the VolumeAttributesClass the PVC currently being reconciled
"%
       | optional
  },

  NFSVolumeSource = {
    path
       | String
       | doc m%"
path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
       | optional,
    server
       | String
       | doc m%"
server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
  },

  Namespace = {
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
       | NamespaceSpec
       | doc m%"
Spec defines the behavior of the Namespace. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | NamespaceStatus
       | doc m%"
Status describes the current status of a Namespace. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  NamespaceCondition = {
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "Human-readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc "Unique, one-word, CamelCase reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of namespace controller condition."
  },

  NamespaceList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Namespace
       | doc m%"
Items is the list of Namespace objects in the list. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
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

  NamespaceSpec = {
    finalizers
       | Array String
       | doc m%"
Finalizers is an opaque list of values that must be empty to permanently remove object from storage. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%
       | optional
  },

  NamespaceStatus = {
    conditions
       | Array NamespaceCondition
       | doc "Represents the latest available observations of a namespace's current state."
       | optional,
    phase
       | String
       | doc m%"
Phase is the current lifecycle phase of the namespace. More info: https://kubernetes.io/docs/tasks/administer-cluster/namespaces/
"%
       | optional
  },

  Node = {
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
       | NodeSpec
       | doc m%"
Spec defines the behavior of a node. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | NodeStatus
       | doc m%"
Most recently observed status of the node. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  NodeAddress = {
    address
       | String
       | doc "The node address.",
    type
       | String
       | doc "Node address type, one of Hostname, ExternalIP or InternalIP."
  },

  NodeAffinity = {
    preferredDuringSchedulingIgnoredDuringExecution
       | Array PreferredSchedulingTerm
       | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
"%
       | optional,
    requiredDuringSchedulingIgnoredDuringExecution
       | NodeSelector
       | doc m%"
If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
"%
       | optional
  },

  NodeCondition = {
    lastHeartbeatTime
       | Time
       | doc "Last time we got an update on a given condition."
       | optional,
    lastTransitionTime
       | Time
       | doc "Last time the condition transit from one status to another."
       | optional,
    message
       | String
       | doc "Human readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc "(brief) reason for the condition's last transition."
       | optional,
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of node condition."
  },

  NodeConfigSource = {
    configMap
       | ConfigMapNodeConfigSource
       | doc "ConfigMap is a reference to a Node's ConfigMap"
       | optional
  },

  NodeConfigStatus = {
    active
       | NodeConfigSource
       | doc m%"
Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
"%
       | optional,
    assigned
       | NodeConfigSource
       | doc m%"
Assigned reports the checkpointed config the node will try to use. When Node.Spec.ConfigSource is updated, the node checkpoints the associated config payload to local disk, along with a record indicating intended config. The node refers to this record to choose its config checkpoint, and reports this record in Assigned. Assigned only updates in the status after the record has been checkpointed to disk. When the Kubelet is restarted, it tries to make the Assigned config the Active config by loading and validating the checkpointed payload identified by Assigned.
"%
       | optional,
    error
       | String
       | doc m%"
Error describes any problems reconciling the Spec.ConfigSource to the Active config. Errors may occur, for example, attempting to checkpoint Spec.ConfigSource to the local Assigned record, attempting to checkpoint the payload associated with Spec.ConfigSource, attempting to load or validate the Assigned config, etc. Errors may occur at different points while syncing config. Earlier errors (e.g. download or checkpointing errors) will not result in a rollback to LastKnownGood, and may resolve across Kubelet retries. Later errors (e.g. loading or validating a checkpointed config) will result in a rollback to LastKnownGood. In the latter case, it is usually possible to resolve the error by fixing the config assigned in Spec.ConfigSource. You can find additional information for debugging by searching the error message in the Kubelet log. Error is a human-readable description of the error state; machines can check whether or not Error is empty, but should not rely on the stability of the Error text across Kubelet versions.
"%
       | optional,
    lastKnownGood
       | NodeConfigSource
       | doc m%"
LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
"%
       | optional
  },

  NodeDaemonEndpoints = {
    kubeletEndpoint
       | DaemonEndpoint
       | doc "Endpoint on which Kubelet is listening."
       | optional
  },

  NodeFeatures = {
    supplementalGroupsPolicy
       | Bool
       | doc m%"
SupplementalGroupsPolicy is set to true if the runtime supports SupplementalGroupsPolicy and ContainerUser.
"%
       | optional
  },

  NodeList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Node
       | doc "List of nodes",
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

  NodeRuntimeHandler = {
    features
       | NodeRuntimeHandlerFeatures
       | doc "Supported features."
       | optional,
    name
       | String
       | doc "Runtime handler name. Empty for the default runtime handler."
       | optional
  },

  NodeRuntimeHandlerFeatures = {
    recursiveReadOnlyMounts
       | Bool
       | doc m%"
RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
"%
       | optional,
    userNamespaces
       | Bool
       | doc m%"
UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
"%
       | optional
  },

  NodeSelector = {
    nodeSelectorTerms
       | Array NodeSelectorTerm
       | doc "Required. A list of node selector terms. The terms are ORed."
  },

  NodeSelectorRequirement = {
    key
       | String
       | doc "The label key that the selector applies to.",
    operator
       | String
       | doc m%"
Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
"%,
    values
       | Array String
       | doc m%"
An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
"%
       | optional
  },

  NodeSelectorTerm = {
    matchExpressions
       | Array NodeSelectorRequirement
       | doc "A list of node selector requirements by node's labels."
       | optional,
    matchFields
       | Array NodeSelectorRequirement
       | doc "A list of node selector requirements by node's fields."
       | optional
  },

  NodeSpec = {
    configSource
       | NodeConfigSource
       | doc m%"
Deprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.
"%
       | optional,
    externalID
       | String
       | doc m%"
Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
"%
       | optional,
    podCIDR
       | String
       | doc "PodCIDR represents the pod IP range assigned to the node."
       | optional,
    podCIDRs
       | Array String
       | doc m%"
podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
"%
       | optional,
    providerID
       | String
       | doc m%"
ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
"%
       | optional,
    taints
       | Array Taint
       | doc "If specified, the node's taints."
       | optional,
    unschedulable
       | Bool
       | doc m%"
Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
"%
       | optional
  },

  NodeStatus = {
    addresses
       | Array NodeAddress
       | doc m%"
List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/reference/node/node-status/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
"%
       | optional,
    allocatable
       | { .. }
       | doc m%"
Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
"%
       | optional,
    capacity
       | { .. }
       | doc m%"
Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity
"%
       | optional,
    conditions
       | Array NodeCondition
       | doc m%"
Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/reference/node/node-status/#condition
"%
       | optional,
    config
       | NodeConfigStatus
       | doc m%"
Status of the config assigned to the node via the dynamic Kubelet config feature.
"%
       | optional,
    daemonEndpoints
       | NodeDaemonEndpoints
       | doc "Endpoints of daemons running on the Node."
       | optional,
    features
       | NodeFeatures
       | doc "Features describes the set of features implemented by the CRI implementation."
       | optional,
    images
       | Array ContainerImage
       | doc "List of container images on this node"
       | optional,
    nodeInfo
       | NodeSystemInfo
       | doc m%"
Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/reference/node/node-status/#info
"%
       | optional,
    phase
       | String
       | doc m%"
NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
"%
       | optional,
    runtimeHandlers
       | Array NodeRuntimeHandler
       | doc "The available runtime handlers."
       | optional,
    volumesAttached
       | Array AttachedVolume
       | doc "List of volumes that are attached to the node."
       | optional,
    volumesInUse
       | Array String
       | doc "List of attachable volumes in use (mounted) by the node."
       | optional
  },

  NodeSwapStatus = {
    capacity
       | Number
       | doc "Total amount of swap memory in bytes."
       | optional
  },

  NodeSystemInfo = {
    architecture
       | String
       | doc "The Architecture reported by the node",
    bootID
       | String
       | doc "Boot ID reported by the node.",
    containerRuntimeVersion
       | String
       | doc m%"
ContainerRuntime Version reported by the node through runtime remote API (e.g. containerd://1.4.2).
"%,
    kernelVersion
       | String
       | doc "Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).",
    kubeProxyVersion
       | String
       | doc "Deprecated: KubeProxy Version reported by the node.",
    kubeletVersion
       | String
       | doc "Kubelet Version reported by the node.",
    machineID
       | String
       | doc m%"
MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
"%,
    operatingSystem
       | String
       | doc "The Operating System reported by the node",
    osImage
       | String
       | doc m%"
OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
"%,
    swap
       | NodeSwapStatus
       | doc "Swap Info reported by the node."
       | optional,
    systemUUID
       | String
       | doc m%"
SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-us/red_hat_subscription_management/1/html/rhsm/uuid
"%
  },

  ObjectFieldSelector = {
    apiVersion
       | String
       | doc "Version of the schema the FieldPath is written in terms of, defaults to \"v1\"."
       | optional,
    fieldPath
       | String
       | doc "Path of the field to select in the specified API version."
  },

  ObjectReference = {
    apiVersion
       | String
       | doc "API version of the referent."
       | optional,
    fieldPath
       | String
       | doc m%"
If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.
"%
       | optional,
    kind
       | String
       | doc m%"
Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    name
       | String
       | doc m%"
Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    namespace
       | String
       | doc m%"
Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
"%
       | optional,
    resourceVersion
       | String
       | doc m%"
Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
"%
       | optional,
    uid
       | String
       | doc m%"
UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
"%
       | optional
  },

  PersistentVolume = {
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
       | PersistentVolumeSpec
       | doc m%"
spec defines a specification of a persistent volume owned by the cluster. Provisioned by an administrator. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes
"%
       | optional,
    status
       | PersistentVolumeStatus
       | doc m%"
status represents the current information/status for the persistent volume. Populated by the system. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistent-volumes
"%
       | optional
  },

  PersistentVolumeClaim = {
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
       | PersistentVolumeClaimSpec
       | doc m%"
spec defines the desired characteristics of a volume requested by a pod author. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
       | optional,
    status
       | PersistentVolumeClaimStatus
       | doc m%"
status represents the current information/status of a persistent volume claim. Read-only. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
       | optional
  },

  PersistentVolumeClaimCondition = {
    lastProbeTime
       | Time
       | doc "lastProbeTime is the time we probed the condition."
       | optional,
    lastTransitionTime
       | Time
       | doc m%"
lastTransitionTime is the time the condition transitioned from one status to another.
"%
       | optional,
    message
       | String
       | doc "message is the human-readable message indicating details about last transition."
       | optional,
    reason
       | String
       | doc m%"
reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "Resizing" that means the underlying persistent volume is being resized.
"%
       | optional,
    status
       | String
       | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=state%20of%20pvc-,conditions.status,-(string)%2C%20required
"%,
    type
       | String
       | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=set%20to%20%27ResizeStarted%27.-,PersistentVolumeClaimCondition,-contains%20details%20about
"%
  },

  PersistentVolumeClaimList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array PersistentVolumeClaim
       | doc m%"
items is a list of persistent volume claims. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
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

  PersistentVolumeClaimSpec = {
    accessModes
       | Array String
       | doc m%"
accessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
"%
       | optional,
    dataSource
       | TypedLocalObjectReference
       | doc m%"
dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
"%
       | optional,
    dataSourceRef
       | TypedObjectReference
       | doc m%"
dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef
  allows any non-core object, as well as PersistentVolumeClaim objects.
* While dataSource ignores disallowed values (dropping them), dataSourceRef
  preserves all values, and generates an error if a disallowed value is
  specified.
* While dataSource only allows local objects, dataSourceRef allows objects
  in any namespaces.
(Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%
       | optional,
    resources
       | VolumeResourceRequirements
       | doc m%"
resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
"%
       | optional,
    selector
       | LabelSelector
       | doc "selector is a label query over volumes to consider for binding."
       | optional,
    storageClassName
       | String
       | doc m%"
storageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
"%
       | optional,
    volumeAttributesClassName
       | String
       | doc m%"
volumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim. If specified, the CSI driver will create or update the volume with the attributes defined in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName, it can be changed after the claim is created. An empty string value means that no VolumeAttributesClass will be applied to the claim but it's not allowed to reset this field to empty string once it is set. If unspecified and the PersistentVolumeClaim is unbound, the default VolumeAttributesClass will be set by the persistentvolume controller if it exists. If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource exists. More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/ (Beta) Using this field requires the VolumeAttributesClass feature gate to be enabled (off by default).
"%
       | optional,
    volumeMode
       | String
       | doc m%"
volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
"%
       | optional,
    volumeName
       | String
       | doc "volumeName is the binding reference to the PersistentVolume backing this claim."
       | optional
  },

  PersistentVolumeClaimStatus = {
    accessModes
       | Array String
       | doc m%"
accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
"%
       | optional,
    allocatedResourceStatuses
       | { .. }
       | doc m%"
allocatedResourceStatuses stores status of resource being resized for the given PVC. Key names follow standard Kubernetes label syntax. Valid values are either:
	* Un-prefixed keys:
		- storage - the capacity of the volume.
	* Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.

ClaimResourceStatus can be in any of following states:
	- ControllerResizeInProgress:
		State set when resize controller starts resizing the volume in control-plane.
	- ControllerResizeFailed:
		State set when resize has failed in resize controller with a terminal error.
	- NodeResizePending:
		State set when resize controller has finished resizing the volume but further resizing of
		volume is needed on the node.
	- NodeResizeInProgress:
		State set when kubelet starts resizing the volume.
	- NodeResizeFailed:
		State set when resizing has failed in kubelet with a terminal error. Transient errors don't set
		NodeResizeFailed.
For example: if expanding a PVC for more capacity - this field can be one of the following states:
	- pvc.status.allocatedResourceStatus['storage'] = "ControllerResizeInProgress"
     - pvc.status.allocatedResourceStatus['storage'] = "ControllerResizeFailed"
     - pvc.status.allocatedResourceStatus['storage'] = "NodeResizePending"
     - pvc.status.allocatedResourceStatus['storage'] = "NodeResizeInProgress"
     - pvc.status.allocatedResourceStatus['storage'] = "NodeResizeFailed"
When this field is not set, it means that no resize operation is in progress for the given PVC.

A controller that receives PVC update with previously unknown resourceName or ClaimResourceStatus should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.

This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
"%
       | optional,
    allocatedResources
       | { .. }
       | doc m%"
allocatedResources tracks the resources allocated to a PVC including its capacity. Key names follow standard Kubernetes label syntax. Valid values are either:
	* Un-prefixed keys:
		- storage - the capacity of the volume.
	* Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.

Capacity reported here may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity.

A controller that receives PVC update with previously unknown resourceName should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.

This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
"%
       | optional,
    capacity
       | { .. }
       | doc "capacity represents the actual resources of the underlying volume."
       | optional,
    conditions
       | Array PersistentVolumeClaimCondition
       | doc m%"
conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'Resizing'.
"%
       | optional,
    currentVolumeAttributesClassName
       | String
       | doc m%"
currentVolumeAttributesClassName is the current name of the VolumeAttributesClass the PVC is using. When unset, there is no VolumeAttributeClass applied to this PersistentVolumeClaim This is a beta field and requires enabling VolumeAttributesClass feature (off by default).
"%
       | optional,
    modifyVolumeStatus
       | ModifyVolumeStatus
       | doc m%"
ModifyVolumeStatus represents the status object of ControllerModifyVolume operation. When this is unset, there is no ModifyVolume operation being attempted. This is a beta field and requires enabling VolumeAttributesClass feature (off by default).
"%
       | optional,
    phase
       | String
       | doc "phase represents the current phase of PersistentVolumeClaim."
       | optional
  },

  PersistentVolumeClaimTemplate = {
    metadata
       | ObjectMeta
       | doc m%"
May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
"%
       | optional,
    spec
       | PersistentVolumeClaimSpec
       | doc m%"
The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
"%
  },

  PersistentVolumeClaimVolumeSource = {
    claimName
       | String
       | doc m%"
claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%,
    readOnly
       | Bool
       | doc "readOnly Will force the ReadOnly setting in VolumeMounts. Default false."
       | optional
  },

  PersistentVolumeList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array PersistentVolume
       | doc m%"
items is a list of persistent volumes. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes
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

  PersistentVolumeSpec = {
    accessModes
       | Array String
       | doc m%"
accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
"%
       | optional,
    awsElasticBlockStore
       | AWSElasticBlockStoreVolumeSource
       | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
       | optional,
    azureDisk
       | AzureDiskVolumeSource
       | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
"%
       | optional,
    azureFile
       | AzureFilePersistentVolumeSource
       | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
"%
       | optional,
    capacity
       | { .. }
       | doc m%"
capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
"%
       | optional,
    cephfs
       | CephFSPersistentVolumeSource
       | doc m%"
cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
"%
       | optional,
    cinder
       | CinderPersistentVolumeSource
       | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    claimRef
       | ObjectReference
       | doc m%"
claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
"%
       | optional,
    csi
       | CSIPersistentVolumeSource
       | doc "csi represents storage that is handled by an external CSI driver."
       | optional,
    fc
       | FCVolumeSource
       | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%
       | optional,
    flexVolume
       | FlexPersistentVolumeSource
       | doc m%"
flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
"%
       | optional,
    flocker
       | FlockerVolumeSource
       | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
"%
       | optional,
    gcePersistentDisk
       | GCEPersistentDiskVolumeSource
       | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
       | optional,
    glusterfs
       | GlusterfsPersistentVolumeSource
       | doc m%"
glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported. More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%
       | optional,
    hostPath
       | HostPathVolumeSource
       | doc m%"
hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
       | optional,
    iscsi
       | ISCSIPersistentVolumeSource
       | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
"%
       | optional,
    local
       | LocalVolumeSource
       | doc "local represents directly-attached storage with node affinity"
       | optional,
    mountOptions
       | Array String
       | doc m%"
mountOptions is the list of mount options, e.g. ["ro", "soft"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
"%
       | optional,
    nfs
       | NFSVolumeSource
       | doc m%"
nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
       | optional,
    nodeAffinity
       | VolumeNodeAffinity
       | doc m%"
nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume.
"%
       | optional,
    persistentVolumeReclaimPolicy
       | String
       | doc m%"
persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
"%
       | optional,
    photonPersistentDisk
       | PhotonPersistentDiskVolumeSource
       | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
"%
       | optional,
    portworxVolume
       | PortworxVolumeSource
       | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate is on.
"%
       | optional,
    quobyte
       | QuobyteVolumeSource
       | doc m%"
quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
"%
       | optional,
    rbd
       | RBDPersistentVolumeSource
       | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported. More info: https://examples.k8s.io/volumes/rbd/README.md
"%
       | optional,
    scaleIO
       | ScaleIOPersistentVolumeSource
       | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
"%
       | optional,
    storageClassName
       | String
       | doc m%"
storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
"%
       | optional,
    storageos
       | StorageOSPersistentVolumeSource
       | doc m%"
storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported. More info: https://examples.k8s.io/volumes/storageos/README.md
"%
       | optional,
    volumeAttributesClassName
       | String
       | doc m%"
Name of VolumeAttributesClass to which this persistent volume belongs. Empty value is not allowed. When this field is not set, it indicates that this volume does not belong to any VolumeAttributesClass. This field is mutable and can be changed by the CSI driver after a volume has been updated successfully to a new class. For an unbound PersistentVolume, the volumeAttributesClassName will be matched with unbound PersistentVolumeClaims during the binding process. This is a beta field and requires enabling VolumeAttributesClass feature (off by default).
"%
       | optional,
    volumeMode
       | String
       | doc m%"
volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
"%
       | optional,
    vsphereVolume
       | VsphereVirtualDiskVolumeSource
       | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
"%
       | optional
  },

  PersistentVolumeStatus = {
    lastPhaseTransitionTime
       | Time
       | doc m%"
lastPhaseTransitionTime is the time the phase transitioned from one to another and automatically resets to current time everytime a volume phase transitions.
"%
       | optional,
    message
       | String
       | doc m%"
message is a human-readable message indicating details about why the volume is in this state.
"%
       | optional,
    phase
       | String
       | doc m%"
phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
"%
       | optional,
    reason
       | String
       | doc m%"
reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
"%
       | optional
  },

  PhotonPersistentDiskVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    pdID
       | String
       | doc "pdID is the ID that identifies Photon Controller persistent disk"
  },

  Pod = {
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
       | PodSpec
       | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | PodStatus
       | doc m%"
Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  PodAffinity = {
    preferredDuringSchedulingIgnoredDuringExecution
       | Array WeightedPodAffinityTerm
       | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
"%
       | optional,
    requiredDuringSchedulingIgnoredDuringExecution
       | Array PodAffinityTerm
       | doc m%"
If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%
       | optional
  },

  PodAffinityTerm = {
    labelSelector
       | LabelSelector
       | doc m%"
A label query over a set of resources, in this case pods. If it's null, this PodAffinityTerm matches with no Pods.
"%
       | optional,
    matchLabelKeys
       | Array String
       | doc m%"
MatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both matchLabelKeys and labelSelector. Also, matchLabelKeys cannot be set when labelSelector isn't set.
"%
       | optional,
    mismatchLabelKeys
       | Array String
       | doc m%"
MismatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both mismatchLabelKeys and labelSelector. Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
"%
       | optional,
    namespaceSelector
       | LabelSelector
       | doc m%"
A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces.
"%
       | optional,
    namespaces
       | Array String
       | doc m%"
namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace".
"%
       | optional,
    topologyKey
       | String
       | doc m%"
This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
"%
  },

  PodAntiAffinity = {
    preferredDuringSchedulingIgnoredDuringExecution
       | Array WeightedPodAffinityTerm
       | doc m%"
The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
"%
       | optional,
    requiredDuringSchedulingIgnoredDuringExecution
       | Array PodAffinityTerm
       | doc m%"
If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
"%
       | optional
  },

  PodCondition = {
    lastProbeTime
       | Time
       | doc "Last time we probed the condition."
       | optional,
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "Human-readable message indicating details about last transition."
       | optional,
    observedGeneration
       | Number
       | doc m%"
If set, this represents the .metadata.generation that the pod condition was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
"%
       | optional,
    reason
       | String
       | doc "Unique, one-word, CamelCase reason for the condition's last transition."
       | optional,
    status
       | String
       | doc m%"
Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%,
    type
       | String
       | doc m%"
Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%
  },

  PodDNSConfig = {
    nameservers
       | Array String
       | doc m%"
A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
"%
       | optional,
    options
       | Array PodDNSConfigOption
       | doc m%"
A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
"%
       | optional,
    searches
       | Array String
       | doc m%"
A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
"%
       | optional
  },

  PodDNSConfigOption = {
    name
       | String
       | doc "Name is this DNS resolver option's name. Required."
       | optional,
    value
       | String
       | doc "Value is this DNS resolver option's value."
       | optional
  },

  PodIP = {
    ip
       | String
       | doc "IP is the IP address assigned to the pod"
  },

  PodList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Pod
       | doc m%"
List of pods. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md
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

  PodOS = {
    name
       | String
       | doc m%"
Name is the name of the operating system. The currently supported values are linux and windows. Additional value may be defined in future and can be one of: https://github.com/opencontainers/runtime-spec/blob/master/config.md#platform-specific-configuration Clients should expect to handle additional values and treat unrecognized values in this field as os: null
"%
  },

  PodReadinessGate = {
    conditionType
       | String
       | doc m%"
ConditionType refers to a condition in the pod's condition list with matching type.
"%
  },

  PodResourceClaim = {
    name
       | String
       | doc m%"
Name uniquely identifies this resource claim inside the pod. This must be a DNS_LABEL.
"%,
    resourceClaimName
       | String
       | doc m%"
ResourceClaimName is the name of a ResourceClaim object in the same namespace as this pod.

Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
"%
       | optional,
    resourceClaimTemplateName
       | String
       | doc m%"
ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this pod.

The template will be used to create a new ResourceClaim, which will be bound to this pod. When this pod is deleted, the ResourceClaim will also be deleted. The pod name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.

This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.

Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
"%
       | optional
  },

  PodResourceClaimStatus = {
    name
       | String
       | doc m%"
Name uniquely identifies this resource claim inside the pod. This must match the name of an entry in pod.spec.resourceClaims, which implies that the string must be a DNS_LABEL.
"%,
    resourceClaimName
       | String
       | doc m%"
ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod. If this is unset, then generating a ResourceClaim was not necessary. The pod.spec.resourceClaims entry can be ignored in this case.
"%
       | optional
  },

  PodSchedulingGate = {
    name
       | String
       | doc "Name of the scheduling gate. Each scheduling gate must have a unique name field."
  },

  PodSecurityContext = {
    appArmorProfile
       | AppArmorProfile
       | doc m%"
appArmorProfile is the AppArmor options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    fsGroup
       | Number
       | doc m%"
A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:

1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----

If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    fsGroupChangePolicy
       | String
       | doc m%"
fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    runAsGroup
       | Number
       | doc m%"
The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    runAsNonRoot
       | Bool
       | doc m%"
Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
       | optional,
    runAsUser
       | Number
       | doc m%"
The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    seLinuxChangePolicy
       | String
       | doc m%"
seLinuxChangePolicy defines how the container's SELinux label is applied to all volumes used by the Pod. It has no effect on nodes that do not support SELinux or to volumes does not support SELinux. Valid values are "MountOption" and "Recursive".

"Recursive" means relabeling of all files on all Pod volumes by the container runtime. This may be slow for large volumes, but allows mixing privileged and unprivileged Pods sharing the same volume on the same node.

"MountOption" mounts all eligible Pod volumes with `-o context` mount option. This requires all Pods that share the same volume to use the same SELinux label. It is not possible to share the same volume among privileged and unprivileged Pods. Eligible volumes are in-tree FibreChannel and iSCSI volumes, and all CSI volumes whose CSI driver announces SELinux support by setting spec.seLinuxMount: true in their CSIDriver instance. Other volumes are always re-labelled recursively. "MountOption" value is allowed only when SELinuxMount feature gate is enabled.

If not specified and SELinuxMount feature gate is enabled, "MountOption" is used. If not specified and SELinuxMount feature gate is disabled, "MountOption" is used for ReadWriteOncePod volumes and "Recursive" for all other volumes.

This field affects only Pods that have SELinux label set, either in PodSecurityContext or in SecurityContext of all containers.

All Pods that use the same volume should use the same seLinuxChangePolicy, otherwise some pods can get stuck in ContainerCreating state. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    seLinuxOptions
       | SELinuxOptions
       | doc m%"
The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    seccompProfile
       | SeccompProfile
       | doc m%"
The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    supplementalGroups
       | Array Number
       | doc m%"
A list of groups applied to the first process run in each container, in addition to the container's primary GID and fsGroup (if specified).  If the SupplementalGroupsPolicy feature is enabled, the supplementalGroupsPolicy field determines whether these are in addition to or instead of any group memberships defined in the container image. If unspecified, no additional groups are added, though group memberships defined in the container image may still be used, depending on the supplementalGroupsPolicy field. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    supplementalGroupsPolicy
       | String
       | doc m%"
Defines how supplemental groups of the first container processes are calculated. Valid values are "Merge" and "Strict". If not specified, "Merge" is used. (Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled and the container runtime must implement support for this feature. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    sysctls
       | Array Sysctl
       | doc m%"
Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    windowsOptions
       | WindowsSecurityContextOptions
       | doc m%"
The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
"%
       | optional
  },

  PodSpec = {
    activeDeadlineSeconds
       | Number
       | doc m%"
Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
"%
       | optional,
    affinity
       | Affinity
       | doc "If specified, the pod's scheduling constraints"
       | optional,
    automountServiceAccountToken
       | Bool
       | doc m%"
AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
"%
       | optional,
    containers
       | Array Container
       | doc m%"
List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
"%,
    dnsConfig
       | PodDNSConfig
       | doc m%"
Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
"%
       | optional,
    dnsPolicy
       | String
       | doc m%"
Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
"%
       | optional,
    enableServiceLinks
       | Bool
       | doc m%"
EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
"%
       | optional,
    ephemeralContainers
       | Array EphemeralContainer
       | doc m%"
List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
"%
       | optional,
    hostAliases
       | Array HostAlias
       | doc m%"
HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.
"%
       | optional,
    hostIPC
       | Bool
       | doc "Use the host's ipc namespace. Optional: Default to false."
       | optional,
    hostNetwork
       | Bool
       | doc m%"
Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
"%
       | optional,
    hostPID
       | Bool
       | doc "Use the host's pid namespace. Optional: Default to false."
       | optional,
    hostUsers
       | Bool
       | doc m%"
Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
"%
       | optional,
    hostname
       | String
       | doc m%"
Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
"%
       | optional,
    imagePullSecrets
       | Array LocalObjectReference
       | doc m%"
ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
"%
       | optional,
    initContainers
       | Array Container
       | doc m%"
List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
"%
       | optional,
    nodeName
       | String
       | doc m%"
NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename
"%
       | optional,
    nodeSelector
       | { .. }
       | doc m%"
NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
"%
       | optional,
    os
       | PodOS
       | doc m%"
Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.

If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions

If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.hostUsers - spec.securityContext.appArmorProfile - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.securityContext.supplementalGroupsPolicy - spec.containers[*].securityContext.appArmorProfile - spec.containers[*].securityContext.seLinuxOptions - spec.containers[*].securityContext.seccompProfile - spec.containers[*].securityContext.capabilities - spec.containers[*].securityContext.readOnlyRootFilesystem - spec.containers[*].securityContext.privileged - spec.containers[*].securityContext.allowPrivilegeEscalation - spec.containers[*].securityContext.procMount - spec.containers[*].securityContext.runAsUser - spec.containers[*].securityContext.runAsGroup
"%
       | optional,
    overhead
       | { .. }
       | doc m%"
Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
"%
       | optional,
    preemptionPolicy
       | String
       | doc m%"
PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.
"%
       | optional,
    "priority"
       | Number
       | doc m%"
The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
"%
       | optional,
    priorityClassName
       | String
       | doc m%"
If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
"%
       | optional,
    readinessGates
       | Array PodReadinessGate
       | doc m%"
If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
"%
       | optional,
    resourceClaims
       | Array PodResourceClaim
       | doc m%"
ResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.

This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.

This field is immutable.
"%
       | optional,
    resources
       | ResourceRequirements
       | doc m%"
Resources is the total amount of CPU and Memory resources required by all containers in the pod. It supports specifying Requests and Limits for "cpu" and "memory" resource names only. ResourceClaims are not supported.

This field enables fine-grained control over resource allocation for the entire pod, allowing resource sharing among containers in a pod.

This is an alpha field and requires enabling the PodLevelResources feature gate.
"%
       | optional,
    restartPolicy
       | String
       | doc m%"
Restart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
"%
       | optional,
    runtimeClassName
       | String
       | doc m%"
RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
"%
       | optional,
    schedulerName
       | String
       | doc m%"
If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
"%
       | optional,
    schedulingGates
       | Array PodSchedulingGate
       | doc m%"
SchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.

SchedulingGates can only be set at pod creation time, and be removed only afterwards.
"%
       | optional,
    securityContext
       | PodSecurityContext
       | doc m%"
SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
"%
       | optional,
    serviceAccount
       | String
       | doc m%"
DeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
"%
       | optional,
    serviceAccountName
       | String
       | doc m%"
ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
"%
       | optional,
    setHostnameAsFQDN
       | Bool
       | doc m%"
If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.
"%
       | optional,
    shareProcessNamespace
       | Bool
       | doc m%"
Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
"%
       | optional,
    subdomain
       | String
       | doc m%"
If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>". If not specified, the pod will not have a domainname at all.
"%
       | optional,
    terminationGracePeriodSeconds
       | Number
       | doc m%"
Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
"%
       | optional,
    tolerations
       | Array Toleration
       | doc "If specified, the pod's tolerations."
       | optional,
    topologySpreadConstraints
       | Array TopologySpreadConstraint
       | doc m%"
TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.
"%
       | optional,
    volumes
       | Array Volume
       | doc m%"
List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
"%
       | optional
  },

  PodStatus = {
    conditions
       | Array PodCondition
       | doc m%"
Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
"%
       | optional,
    containerStatuses
       | Array ContainerStatus
       | doc m%"
Statuses of containers in this pod. Each container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
"%
       | optional,
    ephemeralContainerStatuses
       | Array ContainerStatus
       | doc m%"
Statuses for any ephemeral containers that have run in this pod. Each ephemeral container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
"%
       | optional,
    hostIP
       | String
       | doc m%"
hostIP holds the IP address of the host to which the pod is assigned. Empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns mean that HostIP will not be updated even if there is a node is assigned to pod
"%
       | optional,
    hostIPs
       | Array HostIP
       | doc m%"
hostIPs holds the IP addresses allocated to the host. If this field is specified, the first entry must match the hostIP field. This list is empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns means that HostIPs will not be updated even if there is a node is assigned to this pod.
"%
       | optional,
    initContainerStatuses
       | Array ContainerStatus
       | doc m%"
Statuses of init containers in this pod. The most recent successful non-restartable init container will have ready = true, the most recently started container will have startTime set. Each init container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-and-container-status
"%
       | optional,
    message
       | String
       | doc m%"
A human readable message indicating details about why the pod is in this condition.
"%
       | optional,
    nominatedNodeName
       | String
       | doc m%"
nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
"%
       | optional,
    observedGeneration
       | Number
       | doc m%"
If set, this represents the .metadata.generation that the pod status was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
"%
       | optional,
    phase
       | String
       | doc m%"
The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:

Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.

More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
"%
       | optional,
    podIP
       | String
       | doc m%"
podIP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
"%
       | optional,
    podIPs
       | Array PodIP
       | doc m%"
podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.
"%
       | optional,
    qosClass
       | String
       | doc m%"
The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/#quality-of-service-classes
"%
       | optional,
    reason
       | String
       | doc m%"
A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
"%
       | optional,
    resize
       | String
       | doc m%"
Status of resources resize desired for pod's containers. It is empty if no resources resize is pending. Any changes to container resources will automatically set this to "Proposed" Deprecated: Resize status is moved to two pod conditions PodResizePending and PodResizeInProgress. PodResizePending will track states where the spec has been resized, but the Kubelet has not yet allocated the resources. PodResizeInProgress will track in-progress resizes, and should be present whenever allocated resources != acknowledged resources.
"%
       | optional,
    resourceClaimStatuses
       | Array PodResourceClaimStatus
       | doc "Status of resource claims."
       | optional,
    startTime
       | Time
       | doc m%"
RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
"%
       | optional
  },

  PodTemplate = {
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
    template
       | PodTemplateSpec
       | doc m%"
Template defines the pods that will be created from this pod template. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  PodTemplateList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array PodTemplate
       | doc "List of pod templates",
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

  PodTemplateSpec = {
    metadata
       | ObjectMeta
       | doc m%"
Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | PodSpec
       | doc m%"
Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  PortStatus = {
    error
       | String
       | doc m%"
Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
  CamelCase names
- cloud provider specific error values must have names that comply with the
  format foo.example.com/CamelCase.
"%
       | optional,
    port
       | Number
       | doc "Port is the port number of the service port of which status is recorded here",
    protocol
       | String
       | doc m%"
Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
"%
  },

  PortworxVolumeSource = {
    fsType
       | String
       | doc m%"
fSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    volumeID
       | String
       | doc "volumeID uniquely identifies a Portworx volume"
  },

  PreferredSchedulingTerm = {
    preference
       | NodeSelectorTerm
       | doc "A node selector term, associated with the corresponding weight.",
    weight
       | Number
       | doc m%"
Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
"%
  },

  Probe = {
    exec
       | ExecAction
       | doc "Exec specifies a command to execute in the container."
       | optional,
    failureThreshold
       | Number
       | doc m%"
Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
"%
       | optional,
    grpc
       | GRPCAction
       | doc "GRPC specifies a GRPC HealthCheckRequest."
       | optional,
    httpGet
       | HTTPGetAction
       | doc "HTTPGet specifies an HTTP GET request to perform."
       | optional,
    initialDelaySeconds
       | Number
       | doc m%"
Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
       | optional,
    periodSeconds
       | Number
       | doc m%"
How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
"%
       | optional,
    successThreshold
       | Number
       | doc m%"
Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
"%
       | optional,
    tcpSocket
       | TCPSocketAction
       | doc "TCPSocket specifies a connection to a TCP port."
       | optional,
    terminationGracePeriodSeconds
       | Number
       | doc m%"
Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
"%
       | optional,
    timeoutSeconds
       | Number
       | doc m%"
Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
"%
       | optional
  },

  ProjectedVolumeSource = {
    defaultMode
       | Number
       | doc m%"
defaultMode are the mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    sources
       | Array VolumeProjection
       | doc m%"
sources is the list of volume projections. Each entry in this list handles one source.
"%
       | optional
  },

  QuobyteVolumeSource = {
    group
       | String
       | doc "group to map volume access to Default is no group"
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
"%
       | optional,
    registry
       | String
       | doc m%"
registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
"%,
    tenant
       | String
       | doc m%"
tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
"%
       | optional,
    user
       | String
       | doc "user to map volume access to Defaults to serivceaccount user"
       | optional,
    volume
       | String
       | doc "volume is a string that references an already created Quobyte volume by name."
  },

  RBDPersistentVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
"%
       | optional,
    image
       | String
       | doc m%"
image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    keyring
       | String
       | doc m%"
keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    monitors
       | Array String
       | doc m%"
monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    pool
       | String
       | doc m%"
pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    secretRef
       | SecretReference
       | doc m%"
secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    user
       | String
       | doc m%"
user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional
  },

  RBDVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
"%
       | optional,
    image
       | String
       | doc m%"
image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    keyring
       | String
       | doc m%"
keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    monitors
       | Array String
       | doc m%"
monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%,
    pool
       | String
       | doc m%"
pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional,
    user
       | String
       | doc m%"
user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
"%
       | optional
  },

  ReplicationController = {
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
If the Labels of a ReplicationController are empty, they are defaulted to be the same as the Pod(s) that the replication controller manages. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | ReplicationControllerSpec
       | doc m%"
Spec defines the specification of the desired behavior of the replication controller. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | ReplicationControllerStatus
       | doc m%"
Status is the most recently observed status of the replication controller. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  ReplicationControllerCondition = {
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
       | doc "Type of replication controller condition."
  },

  ReplicationControllerList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ReplicationController
       | doc m%"
List of replication controllers. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller
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

  ReplicationControllerSpec = {
    minReadySeconds
       | Number
       | doc m%"
Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
"%
       | optional,
    replicas
       | Number
       | doc m%"
Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
"%
       | optional,
    selector
       | { .. }
       | doc m%"
Selector is a label query over pods that should match the Replicas count. If Selector is empty, it is defaulted to the labels present on the Pod template. Label keys and values that must match in order to be controlled by this replication controller, if empty defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
"%
       | optional,
    template
       | PodTemplateSpec
       | doc m%"
Template is the object that describes the pod that will be created if insufficient replicas are detected. This takes precedence over a TemplateRef. The only allowed template.spec.restartPolicy value is "Always". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
"%
       | optional
  },

  ReplicationControllerStatus = {
    availableReplicas
       | Number
       | doc m%"
The number of available replicas (ready for at least minReadySeconds) for this replication controller.
"%
       | optional,
    conditions
       | Array ReplicationControllerCondition
       | doc m%"
Represents the latest available observations of a replication controller's current state.
"%
       | optional,
    fullyLabeledReplicas
       | Number
       | doc m%"
The number of pods that have labels matching the labels of the pod template of the replication controller.
"%
       | optional,
    observedGeneration
       | Number
       | doc m%"
ObservedGeneration reflects the generation of the most recently observed replication controller.
"%
       | optional,
    readyReplicas
       | Number
       | doc "The number of ready replicas for this replication controller."
       | optional,
    replicas
       | Number
       | doc m%"
Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
"%
  },

  ResourceClaim = {
    name
       | String
       | doc m%"
Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
"%,
    request
       | String
       | doc m%"
Request is the name chosen for a request in the referenced claim. If empty, everything from the claim is made available, otherwise only the result of this request.
"%
       | optional
  },

  ResourceFieldSelector = {
    containerName
       | String
       | doc "Container name: required for volumes, optional for env vars"
       | optional,
    divisor
       | io.k8s.apimachinery.pkg.api.resource.Quantity
       | doc "Specifies the output format of the exposed resources, defaults to \"1\""
       | optional,
    resource
       | String
       | doc "Required: resource to select"
  },

  ResourceHealth = {
    health
       | String
       | doc m%"
Health of the resource. can be one of:
 - Healthy: operates as normal
 - Unhealthy: reported unhealthy. We consider this a temporary health issue
              since we do not have a mechanism today to distinguish
              temporary and permanent issues.
 - Unknown: The status cannot be determined.
            For example, Device Plugin got unregistered and hasn't been re-registered since.

In future we may want to introduce the PermanentlyUnhealthy Status.
"%
       | optional,
    resourceID
       | String
       | doc m%"
ResourceID is the unique identifier of the resource. See the ResourceID type for more information.
"%
  },

  ResourceQuota = {
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
       | ResourceQuotaSpec
       | doc m%"
Spec defines the desired quota. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | ResourceQuotaStatus
       | doc m%"
Status defines the actual enforced quota and its current usage. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  ResourceQuotaList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ResourceQuota
       | doc m%"
Items is a list of ResourceQuota objects. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
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

  ResourceQuotaSpec = {
    hard
       | { .. }
       | doc m%"
hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
"%
       | optional,
    scopeSelector
       | ScopeSelector
       | doc m%"
scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
"%
       | optional,
    scopes
       | Array String
       | doc m%"
A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
"%
       | optional
  },

  ResourceQuotaStatus = {
    hard
       | { .. }
       | doc m%"
Hard is the set of enforced hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
"%
       | optional,
    used
       | { .. }
       | doc "Used is the current observed total usage of the resource in the namespace."
       | optional
  },

  ResourceRequirements = {
    claims
       | Array ResourceClaim
       | doc m%"
Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container.

This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.

This field is immutable. It can only be set for containers.
"%
       | optional,
    limits
       | { .. }
       | doc m%"
Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
       | optional,
    requests
       | { .. }
       | doc m%"
Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
       | optional
  },

  ResourceStatus = {
    name
       | String
       | doc m%"
Name of the resource. Must be unique within the pod and in case of non-DRA resource, match one of the resources from the pod spec. For DRA resources, the value must be "claim:<claim_name>/<request>". When this status is reported about a container, the "claim_name" and "request" must match one of the claims of this container.
"%,
    resources
       | Array ResourceHealth
       | doc m%"
List of unique resources health. Each element in the list contains an unique resource ID and its health. At a minimum, for the lifetime of a Pod, resource ID must uniquely identify the resource allocated to the Pod on the Node. If other Pod on the same Node reports the status with the same resource ID, it must be the same resource they share. See ResourceID type definition for a specific format it has in various use cases.
"%
       | optional
  },

  SELinuxOptions = {
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
  },

  ScaleIOPersistentVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs"
"%
       | optional,
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
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | SecretReference
       | doc m%"
secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
"%,
    sslEnabled
       | Bool
       | doc m%"
sslEnabled is the flag to enable/disable SSL communication with Gateway, default false
"%
       | optional,
    storageMode
       | String
       | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
"%
       | optional,
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
volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
"%
       | optional
  },

  ScaleIOVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
"%
       | optional,
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
readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
"%,
    sslEnabled
       | Bool
       | doc "sslEnabled Flag enable/disable SSL communication with Gateway, default false"
       | optional,
    storageMode
       | String
       | doc m%"
storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
"%
       | optional,
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
volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
"%
       | optional
  },

  ScopeSelector = {
    matchExpressions
       | Array ScopedResourceSelectorRequirement
       | doc "A list of scope selector requirements by scope of the resources."
       | optional
  },

  ScopedResourceSelectorRequirement = {
    operator
       | String
       | doc m%"
Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
"%,
    scopeName
       | String
       | doc "The name of the scope that the selector applies to.",
    values
       | Array String
       | doc m%"
An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
"%
       | optional
  },

  SeccompProfile = {
    localhostProfile
       | String
       | doc m%"
localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must be set if type is "Localhost". Must NOT be set for any other type.
"%
       | optional,
    type
       | String
       | doc m%"
type indicates which kind of seccomp profile will be applied. Valid options are:

Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
"%
  },

  Secret = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    data
       | { .. }
       | doc m%"
Data contains the secret data. Each key must consist of alphanumeric characters, '-', '_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in https://tools.ietf.org/html/rfc4648#section-4
"%
       | optional,
    immutable
       | Bool
       | doc m%"
Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
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
    stringData
       | { .. }
       | doc m%"
stringData allows specifying non-binary secret data in string form. It is provided as a write-only input field for convenience. All keys and values are merged into the data field on write, overwriting any existing values. The stringData field is never output when reading from the API.
"%
       | optional,
    type
       | String
       | doc m%"
Used to facilitate programmatic handling of secret data. More info: https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
"%
       | optional
  },

  SecretEnvSource = {
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "Specify whether the Secret must be defined"
       | optional
  },

  SecretKeySelector = {
    key
       | String
       | doc "The key of the secret to select from.  Must be a valid secret key.",
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "Specify whether the Secret or its key must be defined"
       | optional
  },

  SecretList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Secret
       | doc m%"
Items is a list of secret objects. More info: https://kubernetes.io/docs/concepts/configuration/secret
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

  SecretProjection = {
    items
       | Array KeyToPath
       | doc m%"
items if unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
"%
       | optional,
    name
       | String
       | doc m%"
Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%
       | optional,
    "optional"
       | Bool
       | doc "optional field specify whether the Secret or its key must be defined"
       | optional
  },

  SecretReference = {
    name
       | String
       | doc "name is unique within a namespace to reference a secret resource."
       | optional,
    namespace
       | String
       | doc "namespace defines the space within which the secret name must be unique."
       | optional
  },

  SecretVolumeSource = {
    defaultMode
       | Number
       | doc m%"
defaultMode is Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
"%
       | optional,
    items
       | Array KeyToPath
       | doc m%"
items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
"%
       | optional,
    "optional"
       | Bool
       | doc "optional field specify whether the Secret or its keys must be defined"
       | optional,
    secretName
       | String
       | doc m%"
secretName is the name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%
       | optional
  },

  SecurityContext = {
    allowPrivilegeEscalation
       | Bool
       | doc m%"
AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    appArmorProfile
       | AppArmorProfile
       | doc m%"
appArmorProfile is the AppArmor options to use by this container. If set, this profile overrides the pod's appArmorProfile. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    capabilities
       | Capabilities
       | doc m%"
The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    privileged
       | Bool
       | doc m%"
Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    procMount
       | String
       | doc m%"
procMount denotes the type of proc mount to use for the containers. The default value is Default which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    readOnlyRootFilesystem
       | Bool
       | doc m%"
Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    runAsGroup
       | Number
       | doc m%"
The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    runAsNonRoot
       | Bool
       | doc m%"
Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
       | optional,
    runAsUser
       | Number
       | doc m%"
The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    seLinuxOptions
       | SELinuxOptions
       | doc m%"
The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    seccompProfile
       | SeccompProfile
       | doc m%"
The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
"%
       | optional,
    windowsOptions
       | WindowsSecurityContextOptions
       | doc m%"
The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
"%
       | optional
  },

  Service = {
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
       | ServiceSpec
       | doc m%"
Spec defines the behavior of a service. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional,
    status
       | ServiceStatus
       | doc m%"
Most recently observed status of the service. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
"%
       | optional
  },

  ServiceAccount = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    automountServiceAccountToken
       | Bool
       | doc m%"
AutomountServiceAccountToken indicates whether pods running as this service account should have an API token automatically mounted. Can be overridden at the pod level.
"%
       | optional,
    imagePullSecrets
       | Array LocalObjectReference
       | doc m%"
ImagePullSecrets is a list of references to secrets in the same namespace to use for pulling any images in pods that reference this ServiceAccount. ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, but ImagePullSecrets are only accessed by the kubelet. More info: https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod
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
    secrets
       | Array ObjectReference
       | doc m%"
Secrets is a list of the secrets in the same namespace that pods running using this ServiceAccount are allowed to use. Pods are only limited to this list if this service account has a "kubernetes.io/enforce-mountable-secrets" annotation set to "true". The "kubernetes.io/enforce-mountable-secrets" annotation is deprecated since v1.32. Prefer separate namespaces to isolate access to mounted secrets. This field should not be used to find auto-generated service account token secrets for use outside of pods. Instead, tokens can be requested directly using the TokenRequest API, or service account token secrets can be manually created. More info: https://kubernetes.io/docs/concepts/configuration/secret
"%
       | optional
  },

  ServiceAccountList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array ServiceAccount
       | doc m%"
List of ServiceAccounts. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
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

  ServiceAccountTokenProjection = {
    audience
       | String
       | doc m%"
audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
"%
       | optional,
    expirationSeconds
       | Number
       | doc m%"
expirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
"%
       | optional,
    path
       | String
       | doc m%"
path is the path relative to the mount point of the file to project the token into.
"%
  },

  ServiceList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array Service
       | doc "List of services",
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

  ServicePort = {
    appProtocol
       | String
       | doc m%"
The application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:

* Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).

* Kubernetes-defined prefixed names:
  * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
  * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
  * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455

* Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.
"%
       | optional,
    name
       | String
       | doc m%"
The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.
"%
       | optional,
    nodePort
       | Number
       | doc m%"
The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
"%
       | optional,
    port
       | Number
       | doc "The port that will be exposed by this service.",
    protocol
       | String
       | doc m%"
The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
"%
       | optional,
    targetPort
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
"%
       | optional
  },

  ServiceSpec = {
    allocateLoadBalancerNodePorts
       | Bool
       | doc m%"
allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.
"%
       | optional,
    clusterIP
       | String
       | doc m%"
clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
"%
       | optional,
    clusterIPs
       | Array String
       | doc m%"
ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs[0] and clusterIP have the same value.

This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
"%
       | optional,
    externalIPs
       | Array String
       | doc m%"
externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
"%
       | optional,
    externalName
       | String
       | doc m%"
externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be "ExternalName".
"%
       | optional,
    externalTrafficPolicy
       | String
       | doc m%"
externalTrafficPolicy describes how nodes distribute service traffic they receive on one of the Service's "externally-facing" addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to "Local", the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get "Cluster" semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.
"%
       | optional,
    healthCheckNodePort
       | Number
       | doc m%"
healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.
"%
       | optional,
    internalTrafficPolicy
       | String
       | doc m%"
InternalTrafficPolicy describes how nodes distribute service traffic they receive on the ClusterIP. If set to "Local", the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features).
"%
       | optional,
    ipFamilies
       | Array String
       | doc m%"
IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services. This field will be wiped when updating a Service to type ExternalName.

This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.
"%
       | optional,
    ipFamilyPolicy
       | String
       | doc m%"
IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.
"%
       | optional,
    loadBalancerClass
       | String
       | doc m%"
loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. "internal-vip" or "example.com/internal-vip". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.
"%
       | optional,
    loadBalancerIP
       | String
       | doc m%"
Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.
"%
       | optional,
    loadBalancerSourceRanges
       | Array String
       | doc m%"
If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/
"%
       | optional,
    ports
       | Array ServicePort
       | doc m%"
The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
"%
       | optional,
    publishNotReadyAddresses
       | Bool
       | doc m%"
publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.
"%
       | optional,
    selector
       | { .. }
       | doc m%"
Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
"%
       | optional,
    sessionAffinity
       | String
       | doc m%"
Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
"%
       | optional,
    sessionAffinityConfig
       | SessionAffinityConfig
       | doc "sessionAffinityConfig contains the configurations of session affinity."
       | optional,
    trafficDistribution
       | String
       | doc m%"
TrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to "PreferClose", implementations should prioritize endpoints that are in the same zone.
"%
       | optional,
    type
       | String
       | doc m%"
type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
"%
       | optional
  },

  ServiceStatus = {
    conditions
       | Array Condition
       | doc "Current service state"
       | optional,
    loadBalancer
       | LoadBalancerStatus
       | doc m%"
LoadBalancer contains the current status of the load-balancer, if one is present.
"%
       | optional
  },

  SessionAffinityConfig = {
    clientIP
       | ClientIPConfig
       | doc "clientIP contains the configurations of Client IP based session affinity."
       | optional
  },

  SleepAction = {
    seconds
       | Number
       | doc "Seconds is the number of seconds to sleep."
  },

  StorageOSPersistentVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | ObjectReference
       | doc m%"
secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
"%
       | optional,
    volumeName
       | String
       | doc m%"
volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
"%
       | optional,
    volumeNamespace
       | String
       | doc m%"
volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
"%
       | optional
  },

  StorageOSVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
"%
       | optional,
    readOnly
       | Bool
       | doc m%"
readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
"%
       | optional,
    secretRef
       | LocalObjectReference
       | doc m%"
secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
"%
       | optional,
    volumeName
       | String
       | doc m%"
volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
"%
       | optional,
    volumeNamespace
       | String
       | doc m%"
volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
"%
       | optional
  },

  Sysctl = {
    name
       | String
       | doc "Name of a property to set",
    value
       | String
       | doc "Value of a property to set"
  },

  TCPSocketAction = {
    host
       | String
       | doc "Optional: Host name to connect to, defaults to the pod IP."
       | optional,
    port
       | io.k8s.apimachinery.pkg.util.intstr.IntOrString
       | doc m%"
Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
"%
  },

  Taint = {
    effect
       | String
       | doc m%"
Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
"%,
    key
       | String
       | doc "Required. The taint key to be applied to a node.",
    timeAdded
       | Time
       | doc m%"
TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints.
"%
       | optional,
    value
       | String
       | doc "The taint value corresponding to the taint key."
       | optional
  },

  Toleration = {
    effect
       | String
       | doc m%"
Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
"%
       | optional,
    key
       | String
       | doc m%"
Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
"%
       | optional,
    operator
       | String
       | doc m%"
Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
"%
       | optional,
    tolerationSeconds
       | Number
       | doc m%"
TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
"%
       | optional,
    value
       | String
       | doc m%"
Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
"%
       | optional
  },

  TopologySelectorLabelRequirement = {
    key
       | String
       | doc "The label key that the selector applies to.",
    values
       | Array String
       | doc m%"
An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
"%
  },

  TopologySelectorTerm = {
    matchLabelExpressions
       | Array TopologySelectorLabelRequirement
       | doc "A list of topology selector requirements by labels."
       | optional
  },

  TopologySpreadConstraint = {
    labelSelector
       | LabelSelector
       | doc m%"
LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
"%
       | optional,
    matchLabelKeys
       | Array String
       | doc m%"
MatchLabelKeys is a set of pod label keys to select the pods over which spreading will be calculated. The keys are used to lookup values from the incoming pod labels, those key-value labels are ANDed with labelSelector to select the group of existing pods over which spreading will be calculated for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector. MatchLabelKeys cannot be set when LabelSelector isn't set. Keys that don't exist in the incoming pod labels will be ignored. A null or empty list means only match against labelSelector.

This is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).
"%
       | optional,
    maxSkew
       | Number
       | doc m%"
MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. The global minimum is the minimum number of matching pods in an eligible domain or zero if the number of eligible domains is less than MinDomains. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 2/2/1: In this case, the global minimum is 1. | zone1 | zone2 | zone3 | |  P P  |  P P  |   P   | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2; scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
"%,
    minDomains
       | Number
       | doc m%"
MinDomains indicates a minimum number of eligible domains. When the number of eligible domains with matching topology keys is less than minDomains, Pod Topology Spread treats "global minimum" as 0, and then the calculation of Skew is performed. And when the number of eligible domains with matching topology keys equals or greater than minDomains, this value has no effect on scheduling. As a result, when the number of eligible domains is less than minDomains, scheduler won't schedule more than maxSkew Pods to those domains. If value is nil, the constraint behaves as if MinDomains is equal to 1. Valid values are integers greater than 0. When value is not nil, WhenUnsatisfiable must be DoNotSchedule.

For example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same labelSelector spread as 2/2/2: | zone1 | zone2 | zone3 | |  P P  |  P P  |  P P  | The number of domains is less than 5(MinDomains), so "global minimum" is treated as 0. In this situation, new pod with the same labelSelector cannot be scheduled, because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones, it will violate MaxSkew.
"%
       | optional,
    nodeAffinityPolicy
       | String
       | doc m%"
NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector when calculating pod topology spread skew. Options are: - Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations. - Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.

If this value is nil, the behavior is equivalent to the Honor policy.
"%
       | optional,
    nodeTaintsPolicy
       | String
       | doc m%"
NodeTaintsPolicy indicates how we will treat node taints when calculating pod topology spread skew. Options are: - Honor: nodes without taints, along with tainted nodes for which the incoming pod has a toleration, are included. - Ignore: node taints are ignored. All nodes are included.

If this value is nil, the behavior is equivalent to the Ignore policy.
"%
       | optional,
    topologyKey
       | String
       | doc m%"
TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. We define a domain as a particular instance of a topology. Also, we define an eligible domain as a domain whose nodes meet the requirements of nodeAffinityPolicy and nodeTaintsPolicy. e.g. If TopologyKey is "kubernetes.io/hostname", each Node is a domain of that topology. And, if TopologyKey is "topology.kubernetes.io/zone", each zone is a domain of that topology. It's a required field.
"%,
    whenUnsatisfiable
       | String
       | doc m%"
WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location,
  but giving higher precedence to topologies that would help reduce the
  skew.
A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
"%
  },

  TypedLocalObjectReference = {
    apiGroup
       | String
       | doc m%"
APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
"%
       | optional,
    kind
       | String
       | doc "Kind is the type of resource being referenced",
    name
       | String
       | doc "Name is the name of resource being referenced"
  },

  TypedObjectReference = {
    apiGroup
       | String
       | doc m%"
APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
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
Namespace is the namespace of resource being referenced Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. (Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
"%
       | optional
  },

  Volume = {
    awsElasticBlockStore
       | AWSElasticBlockStoreVolumeSource
       | doc m%"
awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
"%
       | optional,
    azureDisk
       | AzureDiskVolumeSource
       | doc m%"
azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
"%
       | optional,
    azureFile
       | AzureFileVolumeSource
       | doc m%"
azureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
"%
       | optional,
    cephfs
       | CephFSVolumeSource
       | doc m%"
cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
"%
       | optional,
    cinder
       | CinderVolumeSource
       | doc m%"
cinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
"%
       | optional,
    configMap
       | ConfigMapVolumeSource
       | doc "configMap represents a configMap that should populate this volume"
       | optional,
    csi
       | CSIVolumeSource
       | doc m%"
csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers.
"%
       | optional,
    downwardAPI
       | DownwardAPIVolumeSource
       | doc m%"
downwardAPI represents downward API about the pod that should populate this volume
"%
       | optional,
    emptyDir
       | EmptyDirVolumeSource
       | doc m%"
emptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
"%
       | optional,
    ephemeral
       | EphemeralVolumeSource
       | doc m%"
ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.

Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
   tracking are needed,
c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
   a PersistentVolumeClaim (see EphemeralVolumeSource for more
   information on the connection between this volume type
   and PersistentVolumeClaim).

Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.

Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.

A pod can use both types of ephemeral volumes and persistent volumes at the same time.
"%
       | optional,
    fc
       | FCVolumeSource
       | doc m%"
fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
"%
       | optional,
    flexVolume
       | FlexVolumeSource
       | doc m%"
flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
"%
       | optional,
    flocker
       | FlockerVolumeSource
       | doc m%"
flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
"%
       | optional,
    gcePersistentDisk
       | GCEPersistentDiskVolumeSource
       | doc m%"
gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
"%
       | optional,
    gitRepo
       | GitRepoVolumeSource
       | doc m%"
gitRepo represents a git repository at a particular revision. Deprecated: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
"%
       | optional,
    glusterfs
       | GlusterfsVolumeSource
       | doc m%"
glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported. More info: https://examples.k8s.io/volumes/glusterfs/README.md
"%
       | optional,
    hostPath
       | HostPathVolumeSource
       | doc m%"
hostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
"%
       | optional,
    image
       | ImageVolumeSource
       | doc m%"
image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:

- Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.

The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers[*].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro) and non-executable files (noexec). Sub path mounts for containers are not supported (spec.containers[*].volumeMounts.subpath) before 1.33. The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
"%
       | optional,
    iscsi
       | ISCSIVolumeSource
       | doc m%"
iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
"%
       | optional,
    name
       | String
       | doc m%"
name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
"%,
    nfs
       | NFSVolumeSource
       | doc m%"
nfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
"%
       | optional,
    persistentVolumeClaim
       | PersistentVolumeClaimVolumeSource
       | doc m%"
persistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
"%
       | optional,
    photonPersistentDisk
       | PhotonPersistentDiskVolumeSource
       | doc m%"
photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
"%
       | optional,
    portworxVolume
       | PortworxVolumeSource
       | doc m%"
portworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate is on.
"%
       | optional,
    projected
       | ProjectedVolumeSource
       | doc "projected items for all in one resources secrets, configmaps, and downward API"
       | optional,
    quobyte
       | QuobyteVolumeSource
       | doc m%"
quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
"%
       | optional,
    rbd
       | RBDVolumeSource
       | doc m%"
rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported. More info: https://examples.k8s.io/volumes/rbd/README.md
"%
       | optional,
    scaleIO
       | ScaleIOVolumeSource
       | doc m%"
scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
"%
       | optional,
    secret
       | SecretVolumeSource
       | doc m%"
secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
"%
       | optional,
    storageos
       | StorageOSVolumeSource
       | doc m%"
storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported.
"%
       | optional,
    vsphereVolume
       | VsphereVirtualDiskVolumeSource
       | doc m%"
vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
"%
       | optional
  },

  VolumeDevice = {
    devicePath
       | String
       | doc m%"
devicePath is the path inside of the container that the device will be mapped to.
"%,
    name
       | String
       | doc "name must match the name of a persistentVolumeClaim in the pod"
  },

  VolumeMount = {
    mountPath
       | String
       | doc m%"
Path within the container at which the volume should be mounted.  Must not contain ':'.
"%,
    mountPropagation
       | String
       | doc m%"
mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).
"%
       | optional,
    name
       | String
       | doc "This must match the Name of a Volume.",
    readOnly
       | Bool
       | doc m%"
Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
"%
       | optional,
    recursiveReadOnly
       | String
       | doc m%"
RecursiveReadOnly specifies whether read-only mounts should be handled recursively.

If ReadOnly is false, this field has no meaning and must be unspecified.

If ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.

If this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).

If this field is not specified, it is treated as an equivalent of Disabled.
"%
       | optional,
    subPath
       | String
       | doc m%"
Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
"%
       | optional,
    subPathExpr
       | String
       | doc m%"
Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
"%
       | optional
  },

  VolumeMountStatus = {
    mountPath
       | String
       | doc "MountPath corresponds to the original VolumeMount.",
    name
       | String
       | doc "Name corresponds to the name of the original VolumeMount.",
    readOnly
       | Bool
       | doc "ReadOnly corresponds to the original VolumeMount."
       | optional,
    recursiveReadOnly
       | String
       | doc m%"
RecursiveReadOnly must be set to Disabled, Enabled, or unspecified (for non-readonly mounts). An IfPossible value in the original VolumeMount must be translated to Disabled or Enabled, depending on the mount result.
"%
       | optional
  },

  VolumeNodeAffinity = {
    required
       | NodeSelector
       | doc "required specifies hard node constraints that must be met."
       | optional
  },

  VolumeProjection = {
    clusterTrustBundle
       | ClusterTrustBundleProjection
       | doc m%"
ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.

Alpha, gated by the ClusterTrustBundleProjection feature gate.

ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.

Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
"%
       | optional,
    configMap
       | ConfigMapProjection
       | doc "configMap information about the configMap data to project"
       | optional,
    downwardAPI
       | DownwardAPIProjection
       | doc "downwardAPI information about the downwardAPI data to project"
       | optional,
    secret
       | SecretProjection
       | doc "secret information about the secret data to project"
       | optional,
    serviceAccountToken
       | ServiceAccountTokenProjection
       | doc "serviceAccountToken is information about the serviceAccountToken data to project"
       | optional
  },

  VolumeResourceRequirements = {
    limits
       | { .. }
       | doc m%"
Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
       | optional,
    requests
       | { .. }
       | doc m%"
Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
"%
       | optional
  },

  VsphereVirtualDiskVolumeSource = {
    fsType
       | String
       | doc m%"
fsType is filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
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
  },

  WeightedPodAffinityTerm = {
    podAffinityTerm
       | PodAffinityTerm
       | doc "Required. A pod affinity term, associated with the corresponding weight.",
    weight
       | Number
       | doc m%"
weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
"%
  },

  WindowsSecurityContextOptions = {
    gmsaCredentialSpec
       | String
       | doc m%"
GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
"%
       | optional,
    gmsaCredentialSpecName
       | String
       | doc "GMSACredentialSpecName is the name of the GMSA credential spec to use."
       | optional,
    hostProcess
       | Bool
       | doc m%"
HostProcess determines if a container should be run as a 'Host Process' container. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers). In addition, if HostProcess is true then HostNetwork must also be set to true.
"%
       | optional,
    runAsUserName
       | String
       | doc m%"
The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
"%
       | optional
  }
}