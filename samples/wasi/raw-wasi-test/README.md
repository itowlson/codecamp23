```
rustup target install wasm32-wasi
cargo build --target wasm32-wasi --release
wasmtime run --env THING=Ivan ./target/wasm32-wasi/release/raw-wasi-test.wasm
```
