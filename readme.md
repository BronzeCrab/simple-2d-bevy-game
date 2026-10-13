# Simple 2d Rust Bevy game

## How to run it

```sh
cargo run
```

## How to build for web

setup:

```sh
rustup target add wasm32-unknown-unknown

# wasm-bindgen-cli version must match the wasm-bindgen crate version in Cargo.lock
cargo install -f wasm-bindgen-cli --version <version from Cargo.lock>
```

wasm-opt (Binaryen) is required, install it with any of:

```sh
cargo install wasm-opt

npm install -g binaryen
```

or download from https://github.com/WebAssembly/binaryen/releases

### build

From the root of project you can run the whole pipeline at once:

```powershell
.\scripts\build-web.ps1
```

or do the steps manually:

This command will build the game for the wasm target, creating a binary

```sh
cargo build --profile wasm-release --target wasm32-unknown-unknown
```

Then, wasm-bindgen-cli is used to create javascript bindings to wasm file in the output file
`www/simple_2d_game.js`, which can be loaded using HTML file.

```sh
wasm-bindgen --out-name simple_2d_game --out-dir www --target web target/wasm32-unknown-unknown/wasm-release/simple-2d-bevy-game.wasm
```

Then wasm-opt optimizes the wasm binary (required step, keeps the web build small):

```sh
wasm-opt -Oz -o www/simple_2d_game_bg.wasm www/simple_2d_game_bg.wasm
```

serve it:

```sh
python -m http.server --directory www
```
