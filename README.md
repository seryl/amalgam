# 🔧 Amalgam

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
- 🔍 **Smart Import Resolution** - Automatically resolves K8s type references with proper imports
- 📁 **Package Generation** - Creates organized package structures from multiple CRDs
- 🔌 **Extensible Architecture** - Plugin-based resolver system for adding new type mappings
- 🐙 **GitHub Integration** - Fetch CRDs directly from GitHub repositories

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

### Extensible Resolver System

Add custom type resolvers using the plugin architecture:

```rust
pub trait ReferenceResolver: Send + Sync {
    fn can_resolve(&self, reference: &str) -> bool;
    fn resolve(&self, reference: &str, imports: &[Import], context: &ResolutionContext) -> Option<Resolution>;
    fn name(&self) -> &str;
}
```

Built-in resolvers:
- `KubernetesResolver` - Handles K8s API types
- `LocalTypeResolver` - Resolves local type references
- Easy to add custom resolvers for other type systems

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
│                        amalgam CLI                           │
├─────────────────────────────────────────────────────────────┤
│                    Schema Pipeline                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   CRD    │  │ OpenAPI  │  │    Go    │  │ Protobuf │   │
│  │  Parser  │  │  Parser  │  │   AST    │  │  Parser  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
├─────────────────────────────────────────────────────────────┤
│              Intermediate Representation (IR)                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Unified Type System (Algebraic Types)        │   │
│  │    - Sum Types (Enums/Unions)                        │   │
│  │    - Product Types (Structs/Records)                 │   │
│  │    - Contracts & Refinement Types                    │   │
│  └──────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                   Code Generation                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Nickel  │  │    Go    │  │   CUE    │  │   WASM   │   │
│  │Generator │  │Generator │  │Generator │  │  Module  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## 📂 Project Structure

```
amalgam/
├── Cargo.toml                 # Workspace definition
├── crates/
│   ├── amalgam-core/          # Core IR and type system
│   ├── amalgam-parser/        # Schema parsers (CRD, OpenAPI)
│   ├── amalgam-codegen/       # Code generators with resolver system
│   └── amalgam-cli/           # Command-line interface
├── examples/                  # Example configurations
└── tests/                     # Integration tests
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

# Update snapshots
cargo test -- --ignored
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
