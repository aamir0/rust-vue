[![Build](https://github.com/aamir0/rust-vue/actions/workflows/build.yml/badge.svg)](https://github.com/aamir0/rust-vue/actions/workflows/build.yml)

# rust-vue
Set up monorepo with GitHub Actions using Rust and Vue

## Dependencies

- protoc

## Building

### Rust
```
wasm-pack build --target bundler
```

Protobuf classes available in Cargo `OUT_DIR` (/target/build/rust-vue-xxx/out/snazzy.items.rs)

### Vue
```
protoc --js_out=import_style=commonjs,binary:. src/protobuf/items.proto
```

Protobuf classes available at /src/protobuf/items_pb.js

## Running
```
pnpm dev
```
