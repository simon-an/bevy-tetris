# bevy-tetris

It's Tetris, made with [Bevy](https://github.com/bevyengine/bevy)!

````
RUSTFLAGS=--cfg=web_sys_unstable_apis 
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/bevy-tetris.wasm --out-dir web --no-typescript

wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "tetris" ./target/wasm32-unknown-unknown/release/bevy-tetris.wasm
```
