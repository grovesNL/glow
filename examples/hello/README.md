# How to Build

## Native

To run with glutin and winit:

```shell
cargo run --features=glutin_winit
```

To run with sdl2:

```shell
cargo run --features=sdl2
```

## Web

`cd` to `examples/hello` directory.

The `wasm-bindgen` CLI must match the `wasm-bindgen` crate version in
`Cargo.lock`. Install the matching version with:

```shell
cargo install wasm-bindgen-cli --version <version-from-Cargo.lock>
```

### wasm32

```shell
cargo build --target wasm32-unknown-unknown
mkdir -p generated
wasm-bindgen ../../target/wasm32-unknown-unknown/debug/hello.wasm --out-dir generated --target web
cp index.html generated
```

### wasm64 (memory64)

`wasm64-unknown-unknown` is a tier-3 target, so it requires a nightly toolchain
and a from-source build of `std` (`-Z build-std`):

```shell
rustup toolchain install nightly --component rust-src
cargo +nightly build --target wasm64-unknown-unknown -Z build-std=std,panic_abort
mkdir -p generated
wasm-bindgen ../../target/wasm64-unknown-unknown/debug/hello.wasm --out-dir generated --target web
cp index.html generated
```

Open `generated/index.html` in a browser that supports the WebAssembly
[memory64 proposal](https://caniuse.com/mdn-webassembly_memory64) (recent
versions of Chrome and Firefox enable it by default).
