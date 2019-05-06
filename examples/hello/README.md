# How to Build

## Native

To run with glutin:

```shell
cargo run --features=window-glutin
```

To run with sdl2:

```shell
cargo run --features=window-sdl2
```

## Web

`cd` to `examples/hello` directory

```shell
cargo +nightly build --target wasm32-unknown-unknown
mkdir -p generated
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/hello.wasm --out-dir generated --no-modules
cp index.html generated
```
