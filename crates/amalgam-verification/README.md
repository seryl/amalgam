# amalgam-verification

Comprehensive verification suite for Amalgam-generated Nickel code.

## Overview

This crate provides multi-level validation to prove that generated Nickel types work correctly with real-world Kubernetes/Crossplane/ArgoCD configurations.

## Validation Levels

```
1. Import Resolution    ✓  All imports resolve, no dangling refs
2. Type Checking        ✓  nickel typecheck passes
3. YAML Round-trip      ✓  Nickel → YAML matches original
4. Schema Validation    ✓  kubectl/kubeconform validates
```

## Usage

### As a Library

```rust
use amalgam_verification::Validator;

let validator = Validator::new("path/to/generated");
let report = validator.validate_all()?;

assert!(report.summary.success);
```

### As a Binary

```bash
# Generate validation report
cargo run --bin verification-report -- --path tests/fixtures/generated

# Output to file
cargo run --bin verification-report -- --output report.md

# JSON format
cargo run --bin verification-report -- --json --output report.json

# Skip certain validations
cargo run --bin verification-report -- --skip-typecheck --skip-schema
```

### Run Tests

```bash
# Run all verification tests
cargo test

# Run specific test suite
cargo test binding_resolution
cargo test typecheck
cargo test roundtrip
```

## Test Fixtures

The `tests/fixtures/` directory contains:

- `crds/` - Real CRD sources (k8s, Crossplane, ArgoCD)
- `generated/` - Generated Nickel types from CRDs
- `examples/yaml/` - Reference YAML configurations
- `examples/nickel/` - Nickel equivalents

## What It Validates

### 1. Import Binding Resolution

**Critical for catching the camelCase/PascalCase bug!**

```nickel
// ❌ WRONG - Binding case mismatch
let objectMeta = import "../../k8s_io/v1/ObjectMeta.ncl" in
{ metadata | ObjectMeta | optional }  // ERROR: ObjectMeta undefined!

// ✅ CORRECT - Binding matches usage
let ObjectMeta = import "../../k8s_io/v1/ObjectMeta.ncl" in
{ metadata | ObjectMeta | optional }
```

Validates:
- All import bindings match their usage
- No dangling type references
- Correct PascalCase preservation

### 2. Nickel Type Checking

Runs `nickel typecheck` on all generated files to ensure:
- Type contracts are valid
- No syntax errors
- Cross-references type-check correctly

### 3. YAML Round-trip Testing

Proves Nickel configs are equivalent to YAML originals:

```bash
YAML original → Compare ← Nickel → nickel export → YAML
```

Validates:
- Semantic equivalence (ignores key order, whitespace)
- Correct value mappings
- Complete field coverage

### 4. Schema Validation

Uses `kubeconform` or `kubectl --dry-run` to validate:
- Generated YAML conforms to CRD schemas
- Required fields are present
- Field types are correct

## Report Format

### Markdown

```markdown
# Amalgam Verification Report

**Generated:** 2025-11-17T10:30:00Z

## Summary
✅ **PASS** - All validation levels passed

## Import Resolution
- Files scanned: 247
- Imports found: 1,342
- ✅ Dangling references: 0
- ✅ Case mismatches: 0

## Type Checking
- Files checked: 247
- ✅ Passed: 247/247

## Schema Validation
- Files validated: 5
- ✅ Valid: 5/5
```

### JSON

```json
{
  "timestamp": "2025-11-17T10:30:00Z",
  "summary": {
    "success": true,
    "total_files": 247,
    "duration_ms": 15700
  },
  "binding_resolution": {
    "files_scanned": 247,
    "imports_found": 1342,
    "dangling_references": 0,
    "case_mismatches": 0
  }
}
```

## CI Integration

```yaml
name: Verification
on: [push, pull_request]
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --package amalgam-verification
      - run: cargo run --bin verification-report -- --output report.md
      - uses: actions/upload-artifact@v3
        with:
          name: verification-report
          path: report.md
```

## Development

### Adding New Validators

1. Create module in `src/`
2. Implement validation logic
3. Add to `Validator::validate_all()`
4. Update `ValidationReport`

### Adding Test Fixtures

1. Download CRDs to `tests/fixtures/crds/`
2. Generate types: `amalgam generate ...`
3. Create YAML examples in `tests/fixtures/examples/yaml/`
4. Port to Nickel in `tests/fixtures/examples/nickel/`
5. Add test cases

## Dependencies

### Required

- Rust 1.70+
- serde, serde_yaml, serde_json
- walkdir, glob
- thiserror, anyhow

### Optional

- `nickel` - For type checking (install from https://nickel-lang.org/)
- `kubeconform` - For schema validation without cluster
- `kubectl` - Alternative schema validation (needs cluster access)

## License

MIT OR Apache-2.0
