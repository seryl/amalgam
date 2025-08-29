# Test importing a Crossplane type
let crossplane = import "./crossplane/apiextensions.crossplane.io/v1/composition.ncl" in

{
  test_composition = crossplane.Composition & {
    apiVersion = "apiextensions.crossplane.io/v1",
    kind = "Composition",
    metadata = {
      name = "test-composition",
    },
    spec = {
      compositeTypeRef = {
        apiVersion = "example.com/v1",
        kind = "ExampleResource",
      },
    },
  }
}