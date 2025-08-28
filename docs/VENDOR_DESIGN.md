# Vendor Package Management Design

## Overview

This document outlines the design for distributing and managing generated Nickel type packages, similar to Go's vendor directory or Node's node_modules.

## Package Structure

### Standard Vendor Path
```
project/
├── nickel.toml                 # Project manifest
├── vendor/                      # Vendored packages
│   ├── k8s.io/                # Core Kubernetes types
│   │   ├── manifest.ncl       # Package metadata
│   │   ├── api/
│   │   │   ├── core/
│   │   │   │   ├── v1/
│   │   │   │   │   ├── mod.ncl
│   │   │   │   │   ├── pod.ncl
│   │   │   │   │   ├── container.ncl
│   │   │   │   │   └── ...
│   │   │   └── apps/
│   │   │       └── v1/
│   │   └── apimachinery/
│   │       └── pkg/
│   │           └── apis/
│   │               └── meta/
│   │                   └── v1/
│   ├── crossplane.io/          # Crossplane types
│   │   ├── manifest.ncl
│   │   ├── apiextensions/
│   │   ├── pkg/
│   │   └── ...
│   └── aws.upbound.io/         # AWS Provider types
│       ├── manifest.ncl
│       └── ...
└── main.ncl                    # Your configuration

```

## Package Manifest Format

Each package should have a `manifest.ncl` file:

```nickel
{
  package = {
    name = "crossplane.io",
    version = "1.14.0",
    description = "Crossplane API types for Nickel",
    source = {
      type = "github",
      url = "https://github.com/crossplane/crossplane",
      ref = "v1.14.0",
      path = "cluster/crds",
    },
    generated = {
      tool = "amalgam",
      version = "0.1.0",
      timestamp = "2024-01-15T10:00:00Z",
    },
    dependencies = [
      {
        name = "k8s.io",
        version = ">=1.28.0",
      },
    ],
  },
}
```

## Project Manifest (nickel.toml)

```toml
[project]
name = "my-k8s-config"
version = "0.1.0"

[dependencies]
"k8s.io" = { version = "1.29.0", source = "amalgam:k8s.io" }
"crossplane.io" = { version = "1.14.0", source = "github:crossplane/crossplane" }
"aws.upbound.io" = { version = "0.44.0", source = "github:upbound/provider-aws" }

[dependencies.local]
"custom-crds" = { path = "./crds" }
```

## CLI Commands

### Vendor Management Commands

```bash
# Download and vendor dependencies
amalgam vendor install

# Add a new dependency
amalgam vendor add crossplane.io@1.14.0

# Update all dependencies
amalgam vendor update

# Generate types from a URL and add to vendor
amalgam vendor fetch --url https://github.com/crossplane/crossplane/tree/v1.14.0/cluster/crds

# List vendored packages
amalgam vendor list

# Clean vendor directory
amalgam vendor clean
```

### Import in Nickel Files

```nickel
# Import from vendor directory
let k8s = import "vendor/k8s.io/mod.ncl" in
let crossplane = import "vendor/crossplane.io/mod.ncl" in

# Or with a vendor prefix
let k8s = import "@k8s.io/mod.ncl" in  # @ prefix resolves to vendor/
let crossplane = import "@crossplane.io/mod.ncl" in
```

## Package Registry

### Public Registry
- Host at amalgam package registry or GitHub releases
- Pre-generated common packages (k8s.io, popular CRDs)
- Versioned releases
- Content-addressed storage for immutability

### Package Distribution Formats

1. **Tarball**: `crossplane.io-1.14.0.tar.gz`
2. **Git repository**: Tagged releases
3. **OCI artifacts**: For cloud-native distribution

## Implementation Plan

### Phase 1: Local Vendor Directory
- [x] Generate packages to vendor/ directory
- [x] Support vendor/ imports in examples
- [x] Add manifest.ncl generation
- [x] Basic vendor structure implementation in amalgam-cli

### Phase 2: Package Management
- [x] Implement nickel.toml parser (basic TOML support)
- [x] Add `vendor install` command structure
- [ ] Support full dependency resolution (see [DEPENDENCY_RESOLUTION.md](./DEPENDENCY_RESOLUTION.md))
- [ ] Add dependency version constraints
- [ ] Implement lock file generation

### Phase 3: Package Distribution
- [ ] Create package registry
- [ ] Support downloading from URLs
- [ ] Add package signing/verification

### Phase 4: Advanced Features
- [ ] Dependency version constraints
- [ ] Lock file (nickel.lock)
- [ ] Private registries
- [ ] Caching and offline mode

## Benefits

1. **Reproducible Builds**: Vendor directory ensures consistent dependencies
2. **Offline Development**: All dependencies are local
3. **Version Control**: Can commit vendor/ for guaranteed reproducibility
4. **Type Safety**: Pre-generated types with proper imports
5. **Discoverability**: Easy to browse available types

## Comparison with Other Systems

| Feature | Go Modules | NPM | Our Design |
|---------|------------|-----|------------|
| Vendor directory | ✓ | node_modules | vendor/ |
| Lock file | go.sum | package-lock.json | nickel.lock |
| Registry | proxy.golang.org | npmjs.org | amalgam registry |
| Local packages | replace | file: | path: |
| Version constraints | Semantic | Semantic | Semantic |

## Example Workflow

```bash
# Initialize a new project
$ amalgam init my-project
Created nickel.toml

# Add Crossplane types
$ amalgam vendor add crossplane.io@1.14.0
Fetching crossplane.io@1.14.0...
Generating types...
Added to vendor/crossplane.io/

# Add Kubernetes types (auto-detected as dependency)
$ amalgam vendor install
Resolving dependencies...
  Installing k8s.io@1.29.0 (required by crossplane.io)
Done.

# Your code can now import
$ cat main.ncl
let crossplane = import "vendor/crossplane.io/mod.ncl" in
let k8s = import "vendor/k8s.io/mod.ncl" in
...

# Check in vendor for reproducibility (optional)
$ git add vendor/
$ git commit -m "Add vendored dependencies"
```

## Security Considerations

1. **Package Verification**: Sign packages with GPG/cosign
2. **Checksum Validation**: Verify downloaded packages
3. **Supply Chain**: Track provenance of generated types
4. **Sandboxing**: Run generation in isolated environment

## Next Steps

### Immediate Priorities (Next Sprint)
1. **Implement Version Constraint Parser**: Parse and evaluate semantic version constraints
2. **Build Dependency Graph**: Create data structures for dependency resolution
3. **Add Lock File Support**: Generate and read nickel.lock files
4. **Enhance CLI Output**: Add progress bars and better error messages

### Short-term Goals (1-2 months)
1. **Registry Client**: Implement package fetching from remote registry
2. **Conflict Detection**: Basic conflict detection with clear error messages
3. **Transitive Dependencies**: Support for indirect dependencies
4. **Cache System**: Local cache for downloaded packages

### Long-term Vision (3-6 months)
1. **SAT Solver**: Advanced constraint solving for complex graphs

## Future Extensions

1. **Workspace Support**: Multiple packages in monorepo
2. **Package Publishing**: Push to registry
3. **Diff Tool**: Show changes between versions
4. **Migration Tool**: Update imports when upgrading
5. **IDE Integration**: Auto-complete for vendored packages

## Related Documentation

- [DEPENDENCY_RESOLUTION.md](./DEPENDENCY_RESOLUTION.md) - Detailed design for full dependency resolution
