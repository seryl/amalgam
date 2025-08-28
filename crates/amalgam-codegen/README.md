# amalgam-codegen

Code generation library for amalgam, producing Nickel configurations and Go structs from intermediate representation.

## Overview

`amalgam-codegen` takes the unified type system from `amalgam-core` and generates idiomatic code for target languages.

## Supported Targets

- **Nickel**: Type-safe configuration language with contracts
- **Go**: Structs with JSON tags and validation
- **CUE** (planned): Configuration language
- **WASM** (planned): WebAssembly modules

## Usage

```rust
use amalgam_codegen::{NickelGenerator, GoGenerator};
use amalgam_core::Schema;

// Generate Nickel configuration
let schema = Schema::from_openapi("api.yaml")?;
let nickel_code = NickelGenerator::new()
    .with_imports(true)
    .generate(&schema)?;

// Generate Go structs
let go_code = GoGenerator::new()
    .with_json_tags(true)
    .generate(&schema)?;
```

## Features

- Idempotent code generation
- Preserves documentation and comments
- Automatic import resolution
- Format-aware output (proper indentation)

