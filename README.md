# Snake

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/Christoph-D/snake/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/Christoph-D/snake/tree/main)

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
