# 🔧 Amalgam

[![Crates.io](https://img.shields.io/crates/v/amalgam.svg)](https://crates.io/crates/amalgam)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Generate type-safe [Nickel](https://nickel-lang.org) configurations from any schema source**

Amalgam transforms Kubernetes CRDs, OpenAPI schemas, and other type definitions into strongly-typed Nickel configuration language, enabling type-safe infrastructure as code with automatic validation and completion.

## 🎯 Why Nickel?

[Nickel](https://nickel-lang.org) is a powerful configuration language that offers:
- **Gradual Typing** - Mix static types with dynamic code as needed
- **Contracts** - Runtime validation with custom predicates
- **Merging** - Powerful record merging and extension
- **Functions** - First-class functions for abstraction
- **Correctness** - Designed to prevent configuration errors

Amalgam bridges the gap between existing schemas (K8s CRDs, OpenAPI) and Nickel's type system, giving you the best of both worlds: auto-generated types from authoritative sources with Nickel's powerful configuration capabilities.

## ✨ Features

- 📦 **Import Kubernetes CRDs** - Convert CRDs to strongly-typed [Nickel](https://nickel-lang.org) configurations
- 🔍 **Smart Import Resolution** - Automatically resolves cross-package type references with proper imports
- 📁 **Package Generation** - Creates organized package structures from multiple schemas
- 🔌 **Generic Architecture** - Universal resolver that works with any schema source
- 🐙 **GitHub Integration** - Fetch schemas directly from GitHub repositories

## 📥 Installation

```bash
# Clone the repository
git clone https://github.com/seryl/amalgam
cd amalgam

# Build with Cargo
cargo build --release

# Install locally
cargo install --path crates/amalgam-cli
```

## 🚀 Quick Start

### Import a Single CRD

```bash
# Import from a local file
amalgam import crd --file my-crd.yaml --output my-crd.ncl

# Import from a URL
amalgam import url --url https://raw.githubusercontent.com/example/repo/main/crd.yaml --output output/
```

### Import Crossplane CRDs

```bash
# Fetch all Crossplane CRDs and generate a Nickel package
amalgam import url \
  --url https://github.com/crossplane/crossplane/tree/main/cluster/crds \
  --output crossplane-types/
```

This generates a structured Nickel package:
```
crossplane-types/
├── mod.ncl                                    # Main module
├── apiextensions.crossplane.io/
│   ├── mod.ncl                               # Group module
│   ├── v1/
│   │   ├── mod.ncl                           # Version module
│   │   ├── composition.ncl                   # Type definitions
│   │   └── compositeresourcedefinition.ncl
│   └── v1beta1/
│       └── ...
└── pkg.crossplane.io/
    └── ...
```

## 📝 Generated Nickel Output Example

Amalgam automatically resolves Kubernetes type references and generates clean [Nickel](https://nickel-lang.org) code:

```nickel
# Module: composition.apiextensions.crossplane.io

let k8s_io_v1 = import "../../k8s_io/v1/objectmeta.ncl" in

{
  Composition = {
    apiVersion | optional | String,
    kind | optional | String,
    metadata | optional | k8s_io_v1.ObjectMeta,
    spec | optional | {
      compositeTypeRef | {
        apiVersion | String,
        kind | String,
      },
      # ... more fields
    },
  },
}
```

## 🎯 Key Features Explained

### Import Resolution

The tool intelligently detects and resolves Kubernetes type references:

- **Detects**: `io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta`
- **Generates Import**: `let k8s_io_v1 = import "../../k8s_io/v1/objectmeta.ncl" in`
- **Resolves Reference**: `k8s_io_v1.ObjectMeta`

### Generic Resolver System

The resolver system uses a simple, generic pattern-matching approach that works for any schema source:

```rust
pub struct TypeResolver {
    cache: HashMap<String, Resolution>,
    type_registry: HashMap<String, String>,
}
```

Key features:
- **Universal Pattern Matching** - Works with any schema format (Kubernetes, OpenAPI, Protobuf, etc.)
- **Smart Import Detection** - Automatically identifies when imports are needed based on namespace patterns
- **Type Registry** - Maintains a registry of all known types for accurate resolution
- **Cache-based Performance** - Caches resolutions to avoid repeated lookups
- **No Special-casing** - Generic implementation that doesn't favor any particular schema source

## 💻 CLI Commands

### Main Commands

- `import` - Import types from various sources
  - `crd` - Import from a CRD file
  - `url` - Import from URL (GitHub, raw files)
  - `open-api` - Import from OpenAPI spec
  - `k8s` - Import from Kubernetes cluster (planned)

- `generate` - Generate code from IR
- `convert` - Convert between formats
- `vendor` - Manage vendored packages

### Options

- `-v, --verbose` - Enable verbose output
- `-d, --debug` - Enable debug output with detailed tracing

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        amalgam CLI                          │
├─────────────────────────────────────────────────────────────┤
│                        Schema Pipeline                      │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│   │   CRD    │  │ OpenAPI  │  │    Go    │  │ Protobuf │    │
│   │  Parser  │  │  Parser  │  │   AST    │  │  Parser  │    │
│   └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
├─────────────────────────────────────────────────────────────┤
│               Intermediate Representation (IR)              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Unified Type System (Algebraic Types)        │   │
│  │    - Sum Types (Enums/Unions)                        │   │
│  │    - Product Types (Structs/Records)                 │   │
│  │    - Contracts & Refinement Types                    │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                       Code Generation                       │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│   │  Nickel  │  │    Go    │  │   CUE    │  │   JSON   │    │
│   │Generator │  │Generator │  │Generator │  │ Exporter │    │
│   └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## 📂 Project Structure

```
amalgam/
├── Cargo.toml                 # Workspace definition
├── flake.nix                  # Nix development environment
├── crates/
│   ├── amalgam-core/          # Core IR and type system
│   ├── amalgam-parser/        # Schema parsers (CRD, OpenAPI)
│   ├── amalgam-codegen/       # Code generators with generic resolver
│   ├── amalgam-daemon/        # Runtime daemon for watching changes
│   └── amalgam-cli/           # Command-line interface
├── examples/                  # Example configurations
├── tests/                     # Integration tests
└── docs/                      # Architecture documentation
```

## 💡 Use Cases

### Kubernetes Configuration Management

Generate type-safe [Nickel](https://nickel-lang.org) configurations from your CRDs:

```bash
# Import your custom CRDs
amalgam import crd --file my-operator-crd.yaml --output types/

# Use in Nickel configurations
let types = import "types/my-operator.ncl" in
let config = {
  apiVersion = "example.io/v1",
  kind = "MyResource",
  metadata = {
    name = "example",
  },
  spec = types.MyResourceSpec & {
    # Type-safe configuration with auto-completion
    replicas = 3,
    # ...
  }
}
```

### CrossPlane Composition

Type-safe CrossPlane compositions in Nickel with full IDE support:

```nickel
let crossplane = import "crossplane-types/mod.ncl" in
let composition = crossplane.apiextensions.v1.Composition & {
  metadata.name = "my-composition",
  spec = {
    compositeTypeRef = {
      apiVersion = "example.io/v1",
      kind = "XDatabase",
    },
    # Full type checking and validation
  }
}
```

## 🛠️ Development

### Building

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run with debug logging
cargo run -- --debug import crd --file test.yaml
```

### Testing

The project includes comprehensive test coverage:
- Unit tests for type resolution and parsing
- Integration tests with real CRDs
- Snapshot tests for generated output
- Property-based tests for round-trip conversions

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --package amalgam-parser

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html --output-dir coverage

# Run benchmarks (requires cargo-criterion)
cargo criterion

# Update snapshot tests (requires cargo-insta)
cargo insta review

# Run tests with all features
cargo test --all-features

# Run doctests only
cargo test --doc

# Run a specific test
cargo test test_kubernetes_resolver
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for security vulnerabilities
cargo audit

# Check for outdated dependencies
cargo outdated

# Generate documentation
cargo doc --no-deps --open

# Check licenses
cargo license
```

## 🤝 Contributing

Contributions are welcome! Areas of interest:

- [ ] Additional schema parsers (Protobuf, GraphQL)
- [ ] More code generators (TypeScript, Python)
- [ ] Kubernetes cluster integration
- [ ] Enhanced type inference
- [ ] IDE plugins for generated types

## 📜 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

### Why Apache 2.0?

- ✅ **Enterprise-friendly** - Widely accepted in corporate environments
- ✅ **Patent protection** - Includes express patent grants
- ✅ **Commercial-ready** - Allows building proprietary products and services
- ✅ **Contribution clarity** - Clear terms for contributions

## 🙏 Acknowledgments

- **Generates code for [Nickel](https://nickel-lang.org/)** - A powerful configuration language with contracts and gradual typing
- Inspired by [CUE](https://cuelang.org/) and its approach to configuration
- Uses patterns from [dhall-kubernetes](https://github.com/dhall-lang/dhall-kubernetes)
