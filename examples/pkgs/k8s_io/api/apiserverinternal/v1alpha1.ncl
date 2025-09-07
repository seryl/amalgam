{
  ServerStorageVersion = {
    apiServerID
       | String
       | doc "The ID of the reporting API server."
       | optional,
    decodableVersions
       | Array String
       | doc m%"
The API server can decode objects encoded in these versions. The encodingVersion must be included in the decodableVersions.
"%
       | optional,
    encodingVersion
       | String
       | doc m%"
The API server encodes the object to this version when persisting it in the backend (e.g., etcd).
"%
       | optional,
    servedVersions
       | Array String
       | doc m%"
The API server can serve these versions. DecodableVersions must include all ServedVersions.
"%
       | optional
  },

  StorageVersion = {
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
       | doc "The name is <group>.<resource>."
       | optional,
    spec
       | io.k8s.api.apiserverinternal.v1alpha1.StorageVersionSpec
       | doc "Spec is an empty spec. It is here to comply with Kubernetes API style.",
    status
       | io.k8s.api.apiserverinternal.v1alpha1.StorageVersionStatus
       | doc m%"
API server instances report the version they can decode and the version they encode objects to when persisting objects in the backend.
"%
  },

  StorageVersionCondition = {
    lastTransitionTime
       | Time
       | doc "Last time the condition transitioned from one status to another."
       | optional,
    message
       | String
       | doc "A human readable message indicating details about the transition.",
    observedGeneration
       | Number
       | doc m%"
If set, this represents the .metadata.generation that the condition was set based upon.
"%
       | optional,
    reason
       | String
       | doc "The reason for the condition's last transition.",
    status
       | String
       | doc "Status of the condition, one of True, False, Unknown.",
    type
       | String
       | doc "Type of the condition."
  },

  StorageVersionList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.apiserverinternal.v1alpha1.StorageVersion
       | doc "Items holds a list of StorageVersion",
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

  StorageVersionSpec = {
  },

  StorageVersionStatus = {
    commonEncodingVersion
       | String
       | doc m%"
If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.
"%
       | optional,
    conditions
       | Array io.k8s.api.apiserverinternal.v1alpha1.StorageVersionCondition
       | doc "The latest available observations of the storageVersion's state."
       | optional,
    storageVersions
       | Array io.k8s.api.apiserverinternal.v1alpha1.ServerStorageVersion
       | doc "The reported versions per API server instance."
       | optional
  }
}