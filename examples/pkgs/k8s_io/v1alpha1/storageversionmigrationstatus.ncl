let migrationCondition = import "./MigrationCondition.ncl" in

# Module: k8s.io.v1alpha1

# Status of the storage version migration.
{
  conditions
     | Array migrationCondition
     | doc "The latest available observations of the migration's current state."
     | optional,
  resourceVersion
     | String
     | doc m%"
ResourceVersion to compare with the GC cache for performing the migration. This is the current resource version of given group, version and resource when kube-controller-manager first observes this StorageVersionMigration resource.
"%
     | optional
}