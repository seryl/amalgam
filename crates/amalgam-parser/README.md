# amalgam-parser

Schema parsing library for amalgam, supporting multiple input formats.

## Overview

`amalgam-parser` reads schemas from various sources and converts them to amalgam's unified intermediate representation.

## Supported Formats

- **OpenAPI/Swagger**: v2.0 and v3.0+ specifications
- **Kubernetes CRDs**: Custom Resource Definitions with OpenAPI schemas
- **JSON Schema**: Draft 4, 6, 7, and 2020-12
- **Go Source**: AST parsing of Go structs and interfaces
- **Protocol Buffers** (planned): .proto file parsing

## Usage

```rust
use amalgam_parser::{Parser, CrdParser, OpenApiParser};

// Parse Kubernetes CRDs
let crd_parser = CrdParser::new();
let schema = crd_parser.parse_file("my-crd.yaml")?;

// Parse from live cluster
let schema = crd_parser.parse_from_cluster("my.crd.io", "v1")?;

// Parse OpenAPI spec
let openapi_parser = OpenApiParser::new();
let schema = openapi_parser.parse_file("openapi.yaml")?;
```

## Features

- Automatic format detection
- Schema validation
- Type inference
- Dependency resolution
- Incremental parsing support

