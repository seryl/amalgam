# Verification Plan - Executive Summary

## 🎯 What We're Building

**A comprehensive test suite that proves generated Nickel code works with real-world Kubernetes/Crossplane/ArgoCD configurations.**

## 🔑 Key Deliverables

### 1. Real-World Examples (Concrete Proof)
- ✅ Take ArgoCD Helm chart → Render to YAML → Convert to Nickel → Validate equivalence
- ✅ Take Crossplane Composition YAML → Port to Nickel → Prove it's identical
- ✅ Take k8s Deployment → Show Nickel version works

### 2. Multi-Level Validation
```
Nickel Code
    ↓
1. Import Resolution ✓  (All imports resolve, no dangling refs)
    ↓
2. Type Checking ✓      (nickel typecheck passes)
    ↓
3. YAML Round-trip ✓    (Nickel → YAML matches original)
    ↓
4. Schema Validation ✓  (kubectl/kubeconform validates)
    ↓
5. (Optional) Real Cluster ✓  (Actually apply to k8s)
```

### 3. Automated Test Suite
```bash
$ cargo test --test verification-suite

Running verification tests...
✅ Type generation: 247 types from 3 CRD sources
✅ Import resolution: 0 dangling references
✅ Type checking: 252/252 files passed
✅ Round-trip: 5/5 examples match YAML
✅ Schema validation: 5/5 examples valid

VERIFICATION PASSED ✓
```

## 📁 Project Structure

```
crates/amalgam-verification/
├── tests/fixtures/
│   ├── crds/              # Real CRDs from k8s, Crossplane, ArgoCD
│   ├── generated/         # Generated Nickel types
│   └── examples/
│       ├── yaml/          # Original YAML configs
│       └── nickel/        # Nickel equivalents
│
├── src/
│   ├── nickel_typechecker.rs   # Runs `nickel typecheck`
│   ├── yaml_roundtrip.rs       # Compares YAML semantically
│   ├── schema_validator.rs     # kubectl/kubeconform wrapper
│   └── binding_resolver.rs     # Validates imports
│
└── tests/
    ├── integration_test.rs      # Full end-to-end
    ├── roundtrip_test.rs        # YAML equivalence
    └── binding_test.rs          # Import resolution
```

## 🚀 Implementation Plan

### Phase 1: Setup (Day 1)
**Goal:** Create infrastructure

- Create `amalgam-verification` crate
- Set up fixture directory structure
- Add tool wrappers (nickel, kubectl, kubeconform)

**Output:** Empty test suite ready to populate

---

### Phase 2: CRD Collection & Generation (Day 1-2)
**Goal:** Generate real types from real CRDs

- Download k8s core CRDs (v1.29)
- Download Crossplane CRDs (latest)
- Download ArgoCD CRDs (latest)
- Run `amalgam generate` on each
- Store in `tests/fixtures/generated/`

**Output:** 200+ real Nickel type files

---

### Phase 3: Reference Examples (Day 2-3)
**Goal:** Create concrete examples

**ArgoCD Application:**
```yaml
# examples/yaml/argocd-application.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: guestbook
spec:
  source:
    repoURL: https://github.com/argoproj/argocd-example-apps
    path: guestbook
  destination:
    server: https://kubernetes.default.svc
    namespace: default
```

**Port to Nickel:**
```nickel
# examples/nickel/argocd-application.ncl
let Application = import "../generated/argoproj_io/v1alpha1/Application.ncl" in
let ObjectMeta = import "../generated/k8s_io/v1/ObjectMeta.ncl" in

{
  apiVersion = "argoproj.io/v1alpha1",
  kind = "Application",
  metadata = { name = "guestbook" } | ObjectMeta,
  spec = {
    source = {
      repoURL = "https://github.com/argoproj/argocd-example-apps",
      path = "guestbook",
    },
    destination = {
      server = "https://kubernetes.default.svc",
      namespace = "default",
    },
  },
} | Application
```

**Examples to Create:**
1. ArgoCD Application ✓
2. Crossplane Composition ✓
3. k8s Deployment + Service ✓
4. Cert-manager Certificate ✓
5. ArgoCD Helm chart output ✓ (BIG ONE)

**Output:** 5 YAML files + 5 Nickel files

---

### Phase 4: Validation Tools (Day 3-4)
**Goal:** Build validation infrastructure

**Import Binding Validator:**
```rust
// Validates the critical bug fix!
#[test]
fn test_import_bindings_match_usage() {
    let files = glob("tests/fixtures/generated/**/*.ncl");

    for file in files {
        let bindings = extract_import_bindings(&file);
        let usages = extract_type_usages(&file);

        for (type_name, binding) in bindings {
            assert_eq!(binding, type_name,
                "Case mismatch in {}: '{}' != '{}'",
                file, binding, type_name);
        }
    }
}
```

**Round-trip Tester:**
```rust
#[test]
fn test_argocd_application_roundtrip() {
    let yaml_original = load_yaml("tests/fixtures/examples/yaml/argocd-application.yaml");
    let nickel_file = "tests/fixtures/examples/nickel/argocd-application.ncl";

    // Export Nickel to YAML
    let yaml_from_nickel = run_nickel_export(nickel_file);

    // Compare semantically (ignore key order, whitespace)
    assert_yaml_equivalent(&yaml_original, &yaml_from_nickel);
}
```

**Output:** 4 validation modules implemented

---

### Phase 5: Test Suite (Day 4-5)
**Goal:** Wire everything together

**Test Pyramid:**
```rust
// Level 1: Import resolution
#[test] fn test_all_imports_resolve()
#[test] fn test_no_dangling_references()
#[test] fn test_case_matches_usage()

// Level 2: Type checking
#[test] fn test_typecheck_all_generated()
#[test] fn test_typecheck_examples()

// Level 3: Round-trip
#[test] fn test_argocd_roundtrip()
#[test] fn test_crossplane_roundtrip()
#[test] fn test_k8s_deployment_roundtrip()

// Level 4: Schema validation
#[test] fn test_kubeconform_validates_all()

// Level 5: Integration
#[test] fn test_full_pipeline_e2e()
```

**Output:** 15+ tests, all passing

---

### Phase 6: Automation & Reporting (Day 5)
**Goal:** CI integration and reporting

**GitHub Actions:**
```yaml
name: Verification Suite
on: [push, pull_request]
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --test verification-suite
      - run: cargo run --bin verification-report > report.md
      - uses: actions/upload-artifact@v3
        with:
          name: verification-report
          path: report.md
```

**Report Format:**
```markdown
# Verification Report
✅ PASS - All tests passed

## Details
- CRD Sources: 3
- Types Generated: 247
- Import Checks: 0 errors
- Type Checks: 252/252 passed
- Round-trip Tests: 5/5 matched
- Schema Validation: 5/5 valid
```

**Output:** CI pipeline + markdown reports

---

## 🎯 Success Metrics

### Minimum Viable Product (MVP)
- ✅ 3 CRD sources (k8s, Crossplane, ArgoCD)
- ✅ 5 reference examples (YAML + Nickel)
- ✅ 0 dangling import references
- ✅ 100% type-check pass rate
- ✅ 100% YAML round-trip equivalence
- ✅ Automated test suite runs in CI

### This Proves:
1. ✅ The binding case bug fix works
2. ✅ Generated Nickel types are correct
3. ✅ Real configs can be written in Nickel
4. ✅ Nickel output is equivalent to YAML
5. ✅ The system is production-ready

## 🔧 Key Tools

**External:**
- `nickel` - Type checking and YAML export
- `kubeconform` - Schema validation (no cluster needed)
- `kubectl` - Optional, for real validation

**Rust Crates:**
- `serde_yaml` - YAML parsing/comparison
- `similar` - Diff generation
- `insta` - Snapshot testing

## ⏱️ Timeline

- **Day 1-2:** Setup + CRD generation
- **Day 3-4:** Examples + validation tools
- **Day 4-5:** Test suite + automation
- **Total:** ~5 days for MVP

## 📋 Next Steps

1. **Review this plan** - Confirm approach
2. **Start Phase 1** - Create crate structure
3. **Iterate** - Build incrementally, adjust as needed

## 💡 Why This Matters

This verification suite will:
- ✅ Prove our bug fixes work in production
- ✅ Give confidence to deploy nickel-pkgs
- ✅ Catch regressions automatically
- ✅ Serve as living documentation
- ✅ Enable continuous validation

**Without this:** We can't be sure the generated code actually works.

**With this:** We have concrete proof and ongoing validation.
