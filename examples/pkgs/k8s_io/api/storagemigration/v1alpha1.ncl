{
  GroupVersionResource = {
    group
       | String
       | doc "The name of the group."
       | optional,
    resource
       | String
       | doc "The name of the resource."
       | optional,
    version
       | String
       | doc "The name of the version."
       | optional
  },

  MigrationCondition = {
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
       | doc "Type of the condition."
  },

  StorageVersionMigration = {
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
Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional,
    spec
       | io.k8s.api.storagemigration.v1alpha1.StorageVersionMigrationSpec
       | doc "Specification of the migration."
       | optional,
    status
       | io.k8s.api.storagemigration.v1alpha1.StorageVersionMigrationStatus
       | doc "Status of the migration."
       | optional
  },

  StorageVersionMigrationList = {
    apiVersion
       | String
       | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%
       | optional,
    items
       | Array io.k8s.api.storagemigration.v1alpha1.StorageVersionMigration
       | doc "Items is the list of StorageVersionMigration",
    kind
       | String
       | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%
       | optional,
    metadata
       | ListMeta
       | doc m%"
Standard list metadata More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%
       | optional
  },

  StorageVersionMigrationSpec = {
    continueToken
       | String
       | doc m%"
The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is "Running", users can use this token to check the progress of the migration.
"%
       | optional,
    resource
       | io.k8s.api.storagemigration.v1alpha1.GroupVersionResource
       | doc m%"
The resource that is being migrated. The migrator sends requests to the endpoint serving the resource. Immutable.
"%
  },

  StorageVersionMigrationStatus = {
    conditions
       | Array io.k8s.api.storagemigration.v1alpha1.MigrationCondition
       | doc "The latest available observations of the migration's current state."
       | optional,
    resourceVersion
       | String
       | doc m%"
ResourceVersion to compare with the GC cache for performing the migration. This is the current resource version of given group, version and resource when kube-controller-manager first observes this StorageVersionMigration resource.
"%
       | optional
  }
}