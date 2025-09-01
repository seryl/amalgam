# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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