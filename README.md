# Snake

A simple Snake clone.

## Example

<http://yozora.eu/snake>

## Build (local)

```shell
cargo build
```

## Build (web)

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/snake.wasm
cp -r index.html assets out/
```
