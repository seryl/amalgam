# Test that our naming conventions work correctly:
# - Module files contain all types for that module
# - Import variables use camelCase (e.g., let coreV1 = ...)
# - Types are accessed via dot notation (e.g., coreV1.Container)

# Import the core/v1 module which contains Container and other types
let coreV1 = import "../../pkgs/k8s_io/api/core/v1.ncl" in

# Test that we can use the imported types with dot notation
let testContainer = coreV1.Container & {
  name = "test-container",
  image = "nginx:latest",
  imagePullPolicy = "Always"
} in

# Return a marker to indicate the test passed
{
  naming_test_passed = true,
  container = testContainer
}