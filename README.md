[![Build](https://github.com/aamir0/rust-vue/actions/workflows/build.yml/badge.svg)](https://github.com/aamir0/rust-vue/actions/workflows/build.yml)

# rust-vue
Set up monorepo with GitHub Actions using Rust and Vue

## Building

### Rust
```
wasm-pack build --target bundler
```

Protobuf classes available in Cargo `OUT_DIR` (/target/build/rust-vue-xxx/out/snazzy.items.rs)

### Vue

#### Protobuf
```
pnpm exec protoc --ts_out src/protobuf --proto_path src/protobuf src/protobuf/items.proto
```

Protobuf classes available at /src/protobuf/items.ts

## Running
```
pnpm dev
```
