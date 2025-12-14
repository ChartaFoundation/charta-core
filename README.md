# Charta Core

![CI](https://github.com/ChartaFoundation/charta-mono/workflows/CI/badge.svg)

Core language semantics and IR validation.

## Components

- **IR Schema**: Intermediate Representation type definitions
- **IR Validation**: Schema validation and semantic validation
- **Formal Semantics**: Language semantics (see `semantics.md`)

## Implementation Status

### Completed (Phase 1)

- IR schema types (`src/ir/schema.rs`)
- IR validation against JSON schema (`src/ir/validation.rs`)
- Semantic validation (duplicate names, undefined references)
- Error handling

## Usage

```rust
use charta_core::ir::validation::validate_ir;
use charta_core::ir::schema::load_schema;

// Validate IR against schema
let ir_json = r#"{"version": "0.1.0", "module": {...}}"#;
let schema_path = "spec/ir-schema.json";
let ir = validate_ir(ir_json, schema_path)?;
```

## Testing

```bash
cd charta-core
cargo test
```
