# bevy-tetris

It's Tetris, made with [Bevy](https://github.com/bevyengine/bevy)!

```bash
RUSTFLAGS=--cfg=web_sys_unstable_apis 
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "tetris" ./target/wasm32-unknown-unknown/release/bevy-tetris.wasm
```

# TODO's

- create Transition Component in move and rotate systems

# Current Implementation details:

- Blocks are moved by a MoveEvent
- MoveEvents are created by: Timer, KeyboardInput, and Transition Component, ...

# Game State "Map" Serialization and Display

## Map <> Vec<&str>

A map is Vec<&str> repesentation of the game board. It is used to save the game state to a file.

Empty cells are represented by 'x'.

## Map <> Display/String

Empty Cells are represented by whitespaces ' '. 
First row shows the column numbers.
Columns are separated by '|'.

```text
|0||1||2||3||4||5||6||7||8||9|
| || || || || || || || || || |
```

## Map <> Debug

Each Cell is represented by a tuple (x, y, value).