# Test cross-package imports: CrossPlane importing k8s types
# Validates that cross-package references work with new naming conventions

let composition = import "../../pkgs/crossplane/apiextensions.crossplane.io/crossplane/Composition.ncl" in
let objectMeta = import "../../pkgs/k8s_io/v1/ObjectMeta.ncl" in

# Create a CrossPlane composition that uses k8s metadata
composition & {
  apiVersion = "apiextensions.crossplane.io/v1",
  kind = "Composition",
  metadata = objectMeta & {
    name = "test-composition",
    namespace = "crossplane-system",
    labels = {
      provider = "aws",
      purpose = "test"
    }
  },
  spec = {
    compositeTypeRef = {
      apiVersion = "test.io/v1alpha1",
      kind = "TestResource"
    },
    mode = "Pipeline",
    pipeline = []
  }
}