# amalgam-core

Core intermediate representation (IR) and type system for the amalgam configuration generator.

## Overview

`amalgam-core` provides the foundational type system and intermediate representation used by all amalgam components to translate between different schema and configuration languages.

## Features

- **Unified Type System**: Algebraic data types that can represent concepts from multiple languages
- **Schema IR**: Intermediate representation for schemas from OpenAPI, Kubernetes CRDs, Go types, etc.
- **Type Mapping**: Bidirectional mappings between different type systems
- **Validation Rules**: Contract and refinement type support

## Usage

```rust
use amalgam_core::{Type, Schema, TypeSystem};

// Create a schema from various sources
let schema = Schema::new("MyConfig");

// Add types and constraints
schema.add_type(Type::String)
    .with_constraint(Constraint::MinLength(1))
    .with_constraint(Constraint::Pattern(r"^[a-z]+$"));
```

