# How to Build

## Native

```shell
cargo run
```

## Web

`cd` to `examples/hello` directory

```shell
cargo +nightly build --target wasm32-unknown-unknown
mkdir -p generated
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/hello.wasm --out-dir generated --no-modules
cp index.html generated
```
