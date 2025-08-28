# Examples

This directory contains examples of amalgam usage and generated output.

## crossplane-generated/

Generated Nickel types from Crossplane CRDs with properly resolved Kubernetes type imports.

Generated using:
```bash
amalgam import url \
  --url https://github.com/crossplane/crossplane/tree/main/cluster/crds \
  --output crossplane-generated/
```

### Features Demonstrated:
- ✅ Automatic K8s type detection
- ✅ Import generation with correct paths
- ✅ Reference resolution (e.g., `k8s_io_v1.ObjectMeta` instead of full path)
- ✅ Package structure organization by group/version/kind

### Usage in Nickel:
```nickel
let crossplane = import "crossplane-generated/mod.ncl" in
let composition = crossplane.apiextensions.v1.Composition & {
  metadata.name = "my-composition",
  spec = {
    compositeTypeRef = {
      apiVersion = "example.io/v1",
      kind = "XDatabase",
    },
  }
}
```