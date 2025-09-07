let objectMeta = import "../v1/ObjectMeta.ncl" in
let storageVersionMigrationSpec = import "./StorageVersionMigrationSpec.ncl" in
let storageVersionMigrationStatus = import "./StorageVersionMigrationStatus.ncl" in

# Module: k8s.io.v1alpha1

# StorageVersionMigration represents a migration of stored data to the latest storage version.
{
  apiVersion | optional | String | doc m%"
APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
"%,
  kind | optional | String | doc m%"
Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
"%,
  metadata | optional | objectMeta | doc m%"
Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
"%,
  spec | optional | storageVersionMigrationSpec | doc "Specification of the migration.",
  status | optional | storageVersionMigrationStatus | doc "Status of the migration."
}