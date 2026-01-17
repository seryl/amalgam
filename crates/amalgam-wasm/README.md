# Amalgam WASM

WebAssembly bindings for the Amalgam schema compiler, enabling schema-to-Nickel conversion directly in the browser or Node.js.

## Features

- Convert Kubernetes CRDs to Nickel configurations
- Convert OpenAPI specifications to Nickel
- Create packages from multiple schemas
- Full type resolution and import management
- Zero runtime dependencies

## Installation

### NPM/Yarn

```bash
npm install amalgam-wasm
# or
yarn add amalgam-wasm
```

### Direct Browser Usage

Include the generated `amalgam_wasm.js` and initialize:

```html
<script type="module">
import init, { crd_to_nickel, openapi_to_nickel } from './amalgam_wasm.js';

await init();

// Now you can use the functions
const nickelCode = crd_to_nickel(crdYaml);
</script>
```

## Usage

### Basic CRD Conversion

```javascript
import { crd_to_nickel } from 'amalgam-wasm';

const crdYaml = `
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: samples.example.com
spec:
  # ... CRD spec
`;

// Optional: provide a base module name
const nickelCode = crd_to_nickel(crdYaml, 'example.samples');
// Or let it auto-derive from metadata
const nickelCode = crd_to_nickel(crdYaml);
console.log(nickelCode);
```

### OpenAPI Conversion

```javascript
import { openapi_to_nickel } from 'amalgam-wasm';

const openapiJson = JSON.stringify({
  openapi: "3.0.0",
  // ... OpenAPI spec
});

// Optional: provide a base module name
const nickelCode = openapi_to_nickel(openapiJson, 'myapi');
// Or let it auto-derive from the API title
const nickelCode = openapi_to_nickel(openapiJson);
console.log(nickelCode);
```

### Building Packages

```javascript
import { AmalgamPackage } from 'amalgam-wasm';

const pkg = new AmalgamPackage();

// Add multiple CRDs (with optional base module names)
pkg.add_crd(crd1Yaml);
pkg.add_crd(crd2Yaml, 'custom.module.name');

// Add OpenAPI specs (with optional base module names)
pkg.add_openapi(openapiJson);
pkg.add_openapi(anotherApiJson, 'another.api');

// Generate the complete package
const nickelPackage = pkg.generate();
console.log(`Package contains ${pkg.module_count()} modules`);
console.log(nickelPackage);
```

## Building from Source

```bash
# Install dependencies
cargo install wasm-pack

# Build the WASM package
cd crates/amalgam-wasm
wasm-pack build --target web

# Or use the build script
./scripts/build-wasm.sh
```

## Performance

The WASM module is optimized for size and speed:
- Uses `wee_alloc` for smaller binary size
- Implements panic hooks for better error messages
- Fully streaming parser implementation
- Zero-copy where possible

## Browser Compatibility

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## License

Apache-2.0