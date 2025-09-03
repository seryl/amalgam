let groupversionresource = import "./groupversionresource.ncl" in

# Module: k8s.io.v1alpha1

# Spec of the storage version migration.
{
  continueToken | optional | String | doc m%"
The token used in the list options to get the next chunk of objects to migrate. When the .status.conditions indicates the migration is "Running", users can use this token to check the progress of the migration.
"%,
  resource | optional | groupversionresource | doc m%"
The resource that is being migrated. The migrator sends requests to the endpoint serving the resource. Immutable.
"%
}