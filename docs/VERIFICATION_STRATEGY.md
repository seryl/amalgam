# Amalgam Verification Strategy

## 🎯 Goal

**Prove that generated Nickel types work correctly with real-world Kubernetes configurations.**

We need to validate:
1. ✅ Generated types are correct and complete
2. ✅ Cross-package imports resolve properly
3. ✅ Real configurations can be written in Nickel
4. ✅ Nickel configs are equivalent to YAML originals
5. ✅ Generated code passes schema validation

## 📋 Verification Approach

### Multi-Level Validation Pyramid

```
                    ┌─────────────────────┐
                    │  Real Cluster Test  │  (Optional, smoke tests)
                    │   kubectl apply     │
                    └─────────────────────┘
                            ▲
                            │
                ┌───────────────────────────┐
                │   Schema Validation       │  (kubectl --dry-run, kubeconform)
                │   Validates against CRDs  │
                └───────────────────────────┘
                            ▲
                            │
            ┌───────────────────────────────────┐
            │     YAML Round-trip Testing       │  (Nickel → YAML → Compare)
            │     Semantic equality checks      │
            └───────────────────────────────────┘
                            ▲
                            │
        ┌───────────────────────────────────────────┐
        │        Nickel Type Checking               │  (nickel typecheck)
        │        Static type validation             │
        └───────────────────────────────────────────┘
                            ▲
                            │
    ┌───────────────────────────────────────────────────┐
    │            Import Binding Validation              │  (Parse & verify refs)
    │            All imports resolve correctly          │
    └───────────────────────────────────────────────────┘
                            ▲
                            │
┌───────────────────────────────────────────────────────────┐
│                  Type Generation                          │  (Generate from CRDs)
│                  Creates .ncl files                       │
└───────────────────────────────────────────────────────────┘
```

## 🏗️ Project Structure

```
crates/amalgam-verification/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Public API
│   ├── validator.rs              # Main validation orchestrator
│   ├── nickel_typechecker.rs     # Runs nickel typecheck
│   ├── yaml_roundtrip.rs         # YAML comparison
│   ├── schema_validator.rs       # kubectl/kubeconform integration
│   ├── binding_resolver.rs       # Import resolution checks
│   └── reporters.rs              # Markdown/JSON reports
│
├── tests/
│   ├── fixtures/
│   │   ├── crds/                 # Downloaded CRD sources
│   │   │   ├── k8s-core/         # k8s.io core types
│   │   │   ├── crossplane/       # Crossplane CRDs
│   │   │   ├── argocd/           # ArgoCD CRDs
│   │   │   └── cert-manager/     # Cert-manager CRDs
│   │   │
│   │   ├── generated/            # Generated Nickel types
│   │   │   ├── k8s_io/
│   │   │   ├── apiextensions_crossplane_io/
│   │   │   └── argoproj_io/
│   │   │
│   │   ├── examples/
│   │   │   ├── yaml/             # Reference YAML configs
│   │   │   │   ├── argocd-application.yaml
│   │   │   │   ├── crossplane-composition.yaml
│   │   │   │   ├── k8s-deployment.yaml
│   │   │   │   └── helm-argocd-output.yaml
│   │   │   │
│   │   │   └── nickel/           # Nickel equivalents
│   │   │       ├── argocd-application.ncl
│   │   │       ├── crossplane-composition.ncl
│   │   │       ├── k8s-deployment.ncl
│   │   │       └── helm-argocd-output.ncl
│   │   │
│   │   └── schemas/              # JSON Schemas for validation
│   │
│   ├── integration_test.rs       # Full end-to-end tests
│   ├── type_generation_test.rs   # CRD → Nickel generation
│   ├── binding_resolution_test.rs # Import resolution
│   ├── typecheck_test.rs         # Nickel type checking
│   ├── roundtrip_test.rs         # YAML ↔ Nickel equivalence
│   └── schema_validation_test.rs # kubectl/kubeconform
│
└── README.md                     # Verification guide
```

## 🔬 Test Levels

### Level 1: Type Generation Tests

**Purpose:** Verify CRDs → Nickel generation works

**Test Cases:**
```rust
#[test]
fn test_generate_k8s_core_types() {
    // Generate from k8s core CRDs (Deployment, Service, Pod, etc.)
    let result = generate_from_crd("tests/fixtures/crds/k8s-core/");
    assert!(result.is_ok());

    // Verify key types exist
    assert_file_exists("tests/fixtures/generated/k8s_io/v1/Pod.ncl");
    assert_file_exists("tests/fixtures/generated/apps_v1/Deployment.ncl");
}

#[test]
fn test_generate_crossplane_types() {
    // Generate Crossplane types with k8s references
    let result = generate_from_crd("tests/fixtures/crds/crossplane/");
    assert!(result.is_ok());

    // Verify Composition references ObjectMeta
    let composition = read_file("generated/apiextensions_crossplane_io/v1/Composition.ncl");
    assert!(composition.contains("let ObjectMeta = import"));
}

#[test]
fn test_generate_argocd_types() {
    let result = generate_from_crd("tests/fixtures/crds/argocd/");
    assert!(result.is_ok());
}
```

### Level 2: Import Binding Validation

**Purpose:** Verify all imports resolve, no dangling references

**Test Cases:**
```rust
#[test]
fn test_all_imports_resolve() {
    let validator = BindingResolver::new("tests/fixtures/generated/");
    let report = validator.validate_all();

    assert_eq!(report.dangling_references.len(), 0,
        "Found dangling references: {:?}", report.dangling_references);
}

#[test]
fn test_import_binding_case_matches_usage() {
    // This is our critical bug fix validation!
    let files = glob("tests/fixtures/generated/**/*.ncl");

    for file in files {
        let bindings = extract_import_bindings(&file);
        let usages = extract_type_usages(&file);

        for (type_name, binding) in bindings {
            assert_eq!(binding, type_name,
                "Binding mismatch in {}: '{}' vs '{}'",
                file, binding, type_name);
        }
    }
}

#[test]
fn test_cross_package_imports_correct() {
    // Verify Crossplane → k8s imports use correct paths
    let composition = parse_ncl("generated/apiextensions_crossplane_io/v1/Composition.ncl");

    for import in composition.imports {
        if import.type_name == "ObjectMeta" {
            assert_eq!(import.path, "../../k8s_io/v1/ObjectMeta.ncl");
        }
    }
}
```

### Level 3: Nickel Type Checking

**Purpose:** Verify Nickel's type checker accepts our code

**Test Cases:**
```rust
#[test]
fn test_typecheck_all_generated_types() {
    let files = glob("tests/fixtures/generated/**/*.ncl");

    for file in files {
        let result = run_nickel_typecheck(&file);
        assert!(result.success,
            "Type check failed for {}: {}", file, result.stderr);
    }
}

#[test]
fn test_typecheck_example_configs() {
    let examples = glob("tests/fixtures/examples/nickel/*.ncl");

    for example in examples {
        let result = run_nickel_typecheck(&example);
        assert!(result.success,
            "Example {} failed type check: {}", example, result.stderr);
    }
}
```

### Level 4: YAML Round-trip Testing

**Purpose:** Prove Nickel configs are equivalent to YAML originals

**Test Cases:**
```rust
#[test]
fn test_argocd_application_roundtrip() {
    let yaml_original = read_yaml("tests/fixtures/examples/yaml/argocd-application.yaml");
    let nickel_file = "tests/fixtures/examples/nickel/argocd-application.ncl";

    // Export Nickel → YAML
    let yaml_from_nickel = run_nickel_export(nickel_file);

    // Compare semantically (ignore ordering, whitespace)
    assert_yaml_equivalent(&yaml_original, &yaml_from_nickel);
}

#[test]
fn test_crossplane_composition_roundtrip() {
    let yaml_original = read_yaml("tests/fixtures/examples/yaml/crossplane-composition.yaml");
    let nickel_file = "tests/fixtures/examples/nickel/crossplane-composition.ncl";

    let yaml_from_nickel = run_nickel_export(nickel_file);
    assert_yaml_equivalent(&yaml_original, &yaml_from_nickel);
}

#[test]
fn test_k8s_deployment_roundtrip() {
    let yaml_original = read_yaml("tests/fixtures/examples/yaml/k8s-deployment.yaml");
    let nickel_file = "tests/fixtures/examples/nickel/k8s-deployment.ncl";

    let yaml_from_nickel = run_nickel_export(nickel_file);
    assert_yaml_equivalent(&yaml_original, &yaml_from_nickel);
}

#[test]
fn test_helm_argocd_chart_roundtrip() {
    // This is the BIG test - full Helm chart converted to Nickel
    let yaml_original = read_yaml("tests/fixtures/examples/yaml/helm-argocd-output.yaml");
    let nickel_file = "tests/fixtures/examples/nickel/helm-argocd-output.ncl";

    let yaml_from_nickel = run_nickel_export(nickel_file);
    assert_yaml_equivalent(&yaml_original, &yaml_from_nickel);
}
```

### Level 5: Schema Validation

**Purpose:** Verify exported YAML passes CRD schema validation

**Test Cases:**
```rust
#[test]
fn test_kubectl_validate_examples() {
    let examples = glob("tests/fixtures/examples/nickel/*.ncl");

    for example in examples {
        let yaml = run_nickel_export(&example);

        // Use kubectl --dry-run=server to validate
        let result = run_kubectl_validate(&yaml);
        assert!(result.success,
            "kubectl validation failed for {}: {}", example, result.stderr);
    }
}

#[test]
fn test_kubeconform_validate_examples() {
    // Alternative: use kubeconform (doesn't need cluster)
    let examples = glob("tests/fixtures/examples/nickel/*.ncl");

    for example in examples {
        let yaml = run_nickel_export(&example);
        let result = run_kubeconform(&yaml, "tests/fixtures/schemas/");

        assert!(result.success,
            "Schema validation failed for {}: {}", example, result.stderr);
    }
}
```

### Level 6: Integration Tests

**Purpose:** End-to-end validation of entire workflow

**Test Cases:**
```rust
#[test]
fn test_full_pipeline_argocd() {
    // 1. Download ArgoCD CRDs
    download_crds("https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/crds/");

    // 2. Generate Nickel types
    generate_from_crd("tests/fixtures/crds/argocd/");

    // 3. Verify imports resolve
    assert_no_dangling_references("tests/fixtures/generated/");

    // 4. Type-check generated types
    assert_typecheck_passes("tests/fixtures/generated/");

    // 5. Write example in Nickel
    let example = write_argocd_application();

    // 6. Type-check example
    assert_typecheck_passes(&example);

    // 7. Export to YAML
    let yaml = run_nickel_export(&example);

    // 8. Validate against schema
    assert_schema_valid(&yaml);
}
```

## 📦 Reference Examples

### Example 1: ArgoCD Application

**YAML (Original):**
```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-app
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/example/repo
    targetRevision: HEAD
    path: manifests
  destination:
    server: https://kubernetes.default.svc
    namespace: default
```

**Nickel (Generated Types + Config):**
```nickel
let Application = import "./argoproj_io/v1alpha1/Application.ncl" in
let ObjectMeta = import "./k8s_io/v1/ObjectMeta.ncl" in

{
  apiVersion = "argoproj.io/v1alpha1",
  kind = "Application",
  metadata = {
    name = "my-app",
    namespace = "argocd",
  } | ObjectMeta,
  spec = {
    project = "default",
    source = {
      repoURL = "https://github.com/example/repo",
      targetRevision = "HEAD",
      path = "manifests",
    },
    destination = {
      server = "https://kubernetes.default.svc",
      namespace = "default",
    },
  },
} | Application
```

### Example 2: Crossplane Composition

**YAML:**
```yaml
apiVersion: apiextensions.crossplane.io/v1
kind: Composition
metadata:
  name: my-composition
spec:
  compositeTypeRef:
    apiVersion: example.org/v1
    kind: XMyResource
  resources:
    - name: bucket
      base:
        apiVersion: s3.aws.crossplane.io/v1beta1
        kind: Bucket
```

**Nickel:**
```nickel
let Composition = import "./apiextensions_crossplane_io/v1/Composition.ncl" in
let ObjectMeta = import "./k8s_io/v1/ObjectMeta.ncl" in

{
  apiVersion = "apiextensions.crossplane.io/v1",
  kind = "Composition",
  metadata = {
    name = "my-composition",
  } | ObjectMeta,
  spec = {
    compositeTypeRef = {
      apiVersion = "example.org/v1",
      kind = "XMyResource",
    },
    resources = [
      {
        name = "bucket",
        base = {
          apiVersion = "s3.aws.crossplane.io/v1beta1",
          kind = "Bucket",
        },
      },
    ],
  },
} | Composition
```

## 🚀 Implementation Phases

### Phase 1: Infrastructure Setup (Week 1)
- [ ] Create `amalgam-verification` crate
- [ ] Set up test fixture directory structure
- [ ] Add dependencies (serde_yaml, nickel-lang-core, kube-rs)
- [ ] Create helper functions for running external commands

### Phase 2: CRD Collection & Generation (Week 1)
- [ ] Download k8s core CRDs (v1.29+)
- [ ] Download Crossplane CRDs (latest stable)
- [ ] Download ArgoCD CRDs (latest stable)
- [ ] Generate Nickel types to `tests/fixtures/generated/`
- [ ] Verify generation succeeds

### Phase 3: Validation Tools (Week 2)
- [ ] Implement `BindingResolver` for import validation
- [ ] Implement `NickelTypeChecker` wrapper
- [ ] Implement `YamlRoundTrip` comparator
- [ ] Implement `SchemaValidator` (kubectl/kubeconform)
- [ ] Create `ValidationReport` generator

### Phase 4: Reference Examples (Week 2)
- [ ] Collect real YAML examples
- [ ] Port to Nickel manually
- [ ] Verify they type-check
- [ ] Document the porting process

### Phase 5: Test Suite (Week 3)
- [ ] Write Level 1-3 tests (generation, bindings, type-checking)
- [ ] Write Level 4-5 tests (round-trip, schema validation)
- [ ] Write Level 6 integration tests
- [ ] Ensure all tests pass

### Phase 6: Automation & CI (Week 3)
- [ ] Create `cargo test verification-suite` command
- [ ] Add to GitHub Actions
- [ ] Generate markdown reports
- [ ] Set up notifications

### Phase 7: Documentation (Week 4)
- [ ] Write verification guide
- [ ] Create quickstart tutorial
- [ ] Document common patterns
- [ ] Add troubleshooting section

## 📊 Success Criteria

### Minimum Viable Verification (MVP)
- ✅ 3 real CRD sources (k8s, Crossplane, ArgoCD)
- ✅ 5 reference examples (YAML + Nickel)
- ✅ 0 dangling import references
- ✅ 100% type-check pass rate
- ✅ 100% round-trip equivalence
- ✅ 100% schema validation pass

### Stretch Goals
- ✅ 5 CRD sources (add cert-manager, Istio)
- ✅ 10 reference examples
- ✅ Full Helm chart conversion (ArgoCD chart)
- ✅ Real cluster smoke tests
- ✅ Performance benchmarks

## 🔧 Tools & Dependencies

### External Tools
- **nickel** - Type checking and export
- **kubectl** - Schema validation (optional, needs cluster)
- **kubeconform** - Offline schema validation
- **yq** - YAML manipulation
- **helm** - Chart rendering

### Rust Crates
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
insta = "1.34"  # Snapshot testing
similar = "2.3"  # Diff output
```

## 📈 Metrics & Reporting

### Automated Reports

**Example Output:**
```markdown
# Amalgam Verification Report
Generated: 2025-11-17

## Summary
✅ PASS - All validation levels passed

## Type Generation
- Sources: 3 CRD collections
- Types generated: 247
- Time: 2.3s

## Import Resolution
- Files scanned: 247
- Imports found: 1,342
- Dangling references: 0 ✅
- Case mismatches: 0 ✅

## Type Checking
- Files checked: 247 + 5 examples
- Pass: 252 ✅
- Fail: 0

## Round-trip Testing
- Examples tested: 5
- Equivalent: 5 ✅
- Differences: 0

## Schema Validation
- Examples validated: 5
- Valid: 5 ✅
- Invalid: 0

## Performance
- Total time: 15.7s
- Avg time per file: 62ms
```

## 🎯 Next Steps

1. **Review this plan** - Get feedback on approach
2. **Start Phase 1** - Create crate structure
3. **Implement incrementally** - One phase at a time
4. **Iterate based on findings** - Adjust as needed

This verification system will prove that:
- ✅ Our binding fix works correctly
- ✅ Generated types are production-ready
- ✅ Real-world configs can be written in Nickel
- ✅ The ecosystem is ready for nickel-pkgs deployment
