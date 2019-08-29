# CHIP-8 WASM

CHIP-8 emulator compiled to WebAssembly to play CHIP-8 games in a web browser

## Build

Make sure you have [Rust](https://www.rust-lang.org/tools/install), [wasm-pack](https://rustwasm.github.io/wasm-pack/installer) and [Node.js](https://nodejs.org/en/) installed. Running `rustc -V` should print at least 1.30.0.

Run a local build with:
```
npm install
npm run serve
```

Visit http://localhost:8080 to play!
