# Test that our naming conventions work correctly:
# - PascalCase for file names (e.g., ObjectMeta.ncl)
# - camelCase for import variables (e.g., let objectMeta = ...)

# Import with camelCase variable names from PascalCase files
let objectMeta = import "../../pkgs/k8s_io/v1/ObjectMeta.ncl" in
let managedFieldsEntry = import "../../pkgs/k8s_io/v1/ManagedFieldsEntry.ncl" in
let ownerReference = import "../../pkgs/k8s_io/v1/OwnerReference.ncl" in

# Test that we can use the imported types with camelCase variables
let testMeta = objectMeta & {
  name = "test-resource",
  namespace = "default",
  managedFields = [
    managedFieldsEntry & {
      manager = "test-manager",
      operation = "Update",
      apiVersion = "v1"
    }
  ],
  ownerReferences = [
    ownerReference & {
      apiVersion = "v1",
      kind = "Pod",
      name = "owner-pod",
      uid = "12345"
    }
  ]
} in

# Return a marker to indicate the test passed
{
  naming_test_passed = true,
  metadata = testMeta
}