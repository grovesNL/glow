# How to Build

## Native

To run with glutin:

```shell
cargo run --features=glutin
```

To run with sdl2:

```shell
cargo run --features=sdl2
```

## Web

`cd` to `examples/hello` directory

To run with web-sys:

```shell
cargo build --target wasm32-unknown-unknown
mkdir -p generated
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/hello.wasm --out-dir generated --target web
cp index.html generated
```
