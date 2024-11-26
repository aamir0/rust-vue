[![Build](https://github.com/aamir0/rust-vue/actions/workflows/build.yml/badge.svg)](https://github.com/aamir0/rust-vue/actions/workflows/build.yml)

# rust-vue
Set up monorepo with GitHub Actions using Rust and Vue

## Building

### Rust
```
wasm-pack build --target bundler
```

Protobuf classes available in Cargo `OUT_DIR` (/target/build/rust-vue-xxx/out/snazzy.items.rs)

## Running
```
pnpm dev
```
