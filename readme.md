# Simple 2d Rust Bevy game

## How to run it

```sh
cargo run
```

## how to build for web

setup:

```sh
rustup target add wasm32-unknown-unknown

cargo install wasm-bindgen-cli

```

form the root of project:
This command will build the example for the wasm target, creating a binary

```sh
cargo build --profile wasm-release --target wasm32-unknown-unknown
```

Then, wasm-bindgen-cli is used to create javascript bindings to wasm file in the output file
www/simple_2d_game.js, which can be loaded using HTML file.

```sh
wasm-bindgen --out-name simple_2d_game --out-dir www --target web target/wasm32-unknown-unknown/wasm-release/simple-2d-bevy-game.wasm
```

serve it:

```sh
python -m http.server --directory www
```
