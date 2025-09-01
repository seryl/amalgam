# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.3] - 2025-09-01

### Added
- **Recursive Type Discovery**: Replaced hardcoded namespace lists with intelligent recursive discovery that automatically finds all referenced types
- **Comprehensive Type Coverage**: Expanded from 199 to 321 Kubernetes types through recursive discovery including versioned APIs (v1alpha1, v1beta1, v2)
- Support for unversioned k8s types (e.g., `RawExtension`, `IntOrString`) placed in v0 directory to avoid conflicts
- Reserved keyword escaping for field names starting with `$` (like `$ref`, `$schema`) in generated Nickel code

### Fixed  
- **Required Field Usability Issue**: Made all fields optional by default to enable gradual object construction (e.g., `k8s.v1.LabelSelector & {}` now works)
- **Cross-Package Import Resolution**: Fixed imports to use full package IDs from manifest configuration instead of bare package names
- Missing type references (e.g., `RawExtension`, `NodeSelector`) now properly discovered and generated
- Cross-version imports for unversioned runtime types (v0 → other versions)
- Syntax errors from unescaped special field names in JSON Schema types
- Reserved keyword escaping in JSON object field names within default values

### Changed
- **Breaking**: All generated fields are now optional by default instead of required, enabling practical usage patterns
- k8s type extraction now uses seed-based recursive discovery instead of fixed namespace lists
- Updated to Kubernetes v1.33.4 schema version (from v1.31.0)  
- Unversioned types are placed in v0 to distinguish from versioned APIs
- Enhanced import logic handles both v1 core types and v0 unversioned types
- Package imports now use full package IDs like `"github:seryl/nickel-pkgs/pkgs/k8s_io"` for consistency

## [0.6.2] - 2025-09-01

### Fixed
- Fixed missing cross-version imports in k8s_io packages (e.g., v1alpha1/v1beta1 types now properly import ObjectMeta, Condition, etc. from v1)
- Consolidated duplicate `handle_k8s_core_import` implementations between lib.rs and main.rs

### Added
- Test suite for cross-version k8s type imports to prevent regression

## [0.6.1] - 2025-09-01

### Added
- Smart workspace dependency management tool (`workspace-deps`) for switching between local and remote dependencies
- Python-based version bump tool (`version-bump`) for reliable version management
- Python-based tooling infrastructure in `nix/packages/` for better maintainability
- SmartError pattern for actionable error messages in development tools

### Changed
- Replaced hardcoded dev-mode script with dynamic workspace dependency discovery
- Updated release script to use new Python-based version-bump tool
- Improved development environment by removing unnecessary cargo tools (cargo-workspaces, cargo-release)
- Fixed Python linting issues by using shell wrapper approach

### Fixed
- Test fixture inclusion in Nix builds with custom source filter
- Workspace dependency publishing workflow for crates.io
- Release script version bumping now works correctly

## [0.6.0] - 2025-09-01

### Changed
- **BREAKING**: Changed package output directory from `packages/` to `pkgs/`
- **BREAKING**: Updated base package ID to `github:seryl/nickel-pkgs/pkgs` to match nickel-pkgs structure
- Updated manifest to use latest stable versions: Kubernetes v1.33.4, Crossplane v2.0.2
- Validation now exclusively uses the Nickel CLI binary, not the library
- nickel-with-packages is exposed in flake for external users

### Fixed
- Removed nickel-lang-core dependency to avoid API instability issues
- Simplified Nickel validation tests to use CLI instead of library API
- Fixed compilation errors with external projects using amalgam
- Enhanced fingerprinting to properly detect version changes in manifests
- URL sources now properly lock to specific versions instead of tracking main branch

## [0.5.1] - 2025-09-01

### Added
- `--version` flag to display amalgam version information
- Workspace metadata for crane to silence warnings
- CI can now be disabled via GitHub repository variable `ENABLE_CI` (disabled by default)

### Fixed
- Updated GitHub Actions to use latest versions (upload-artifact v4, install-nix-action v30)
- Resolved all clippy warnings and formatting issues
- Added missing k8s-openapi dependency to amalgam-daemon
- Fixed redundant guards and iterator usage in dependency analyzer
- Corrected file extension checking to use `is_some_and`

### Changed
- Made CLI command optional to support version flag without subcommand

## [0.5.0] - 2025-09-01

### Added
- GitHub Actions workflow for Nix flake CI/CD with devshell integration
- Support for `--package-path` option in validation command for local package resolution
- Generic package dependency resolution using manifest-based registry
- Git ref tracking for URL-based package sources (e.g., `v1.17.2`)
- Package fingerprinting with SHA256 for intelligent change detection
- Comprehensive test suite for manifest generation
- Library interface (`lib.rs`) for better testing support

### Changed
- **BREAKING**: Package dependencies now always use Index format instead of Path
  - All packages reference upstream repositories via `'Index { package = "...", version = "..." }`
  - Removed special-case handling for k8s_io, crossplane, and other packages
- Updated reqwest from 0.11 to 0.12 (workspace-wide)
- Improved manifest generation with metadata preservation in comments
- Validation now uses absolute paths for nickel binary resolution
- CI pipeline now uses `ci-runner` command from devshell

### Fixed
- Duplicate hyper and h2 dependency versions resolved
- Reserved keyword escaping in generated Nickel code
- Import resolution for k8s core types
- Package validation with proper NICKEL_IMPORT_PATH environment variable

### Removed
- Hardcoded package ID generation for specific packages
- Local Path dependencies in favor of Index dependencies
- Unused test directories and duplicate files in examples/

## [0.4.1] - 2025-08-28

### Initial Features
- Core amalgam functionality for generating Nickel packages from:
  - Kubernetes CRDs
  - OpenAPI specifications
  - Go type definitions
- Manifest-based package generation system
- Incremental compilation with fingerprinting
- Support for complex type mappings and imports
- Package validation with Nickel typecheck
- Vendor management system