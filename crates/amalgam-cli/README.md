# amalgam

Type-safe configuration generator for Nickel from various schema sources.

## Overview

`amalgam` is a command-line tool that generates type-safe [Nickel](https://nickel-lang.org) configurations from Kubernetes CRDs, OpenAPI specifications, Go types, and other schema sources.

## Installation

```bash
cargo install amalgam
```

Or with Nix:
```bash
nix run github:seryl/amalgam
```

## Usage

### Import Kubernetes CRDs

```bash
# Import from live cluster
amalgam k8s-import --context production --output ./k8s-types

# Import specific CRD
amalgam k8s-import --crd cert-manager.io --output ./cert-manager

# Import from file
amalgam import --input my-crd.yaml --output ./types
```

### Convert OpenAPI to Nickel

```bash
amalgam import --input openapi.yaml --output ./api-types --format nickel
```

### Generate Go structs from Nickel

```bash
amalgam export --input config.ncl --output types.go --format go
```

### Watch mode

```bash
# Watch for changes and regenerate
amalgam watch --input ./schemas --output ./generated
```

## Features

- **Multi-format Support**: OpenAPI, Kubernetes CRDs, JSON Schema, Go AST
- **Bidirectional**: Import to Nickel, export from Nickel
- **Type Safety**: Generates contracts and validation
- **Dependency Resolution**: Automatic import management
- **Incremental**: Only regenerates changed schemas

## Configuration

Create an `amalgam.toml` file:

```toml
[input]
paths = ["./schemas", "./crds"]
watch = true

[output]
path = "./generated"
format = "nickel"

[kubernetes]
enabled = true
context = "default"
namespaces = ["default", "kube-system"]
```

## Examples

See the [examples](https://github.com/seryl/amalgam/tree/main/examples) directory for sample configurations and generated types.

